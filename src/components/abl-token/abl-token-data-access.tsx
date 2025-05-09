'use client'

import { getABLTokenProgram, getABLTokenProgramId } from '@project/anchor'
import { useConnection } from '@solana/wallet-adapter-react'
import { Cluster, PublicKey } from '@solana/web3.js'
import { useMutation, useQuery } from '@tanstack/react-query'
import { useMemo } from 'react'
import { useCluster } from '../cluster/cluster-data-access'
import { useAnchorProvider } from '../solana/solana-provider'
import { useTransactionToast } from '../use-transaction-toast'
import { toast } from 'sonner'
import { BN } from '@coral-xyz/anchor'

export function useAblTokenProgram() {
  const { connection } = useConnection()
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const provider = useAnchorProvider()
  const programId = useMemo(() => getABLTokenProgramId(cluster.network as Cluster), [cluster])
  const program = useMemo(() => getABLTokenProgram(provider, programId), [provider, programId])

  const getProgramAccount = useQuery({
    queryKey: ['get-program-account', { cluster }],
    queryFn: () => connection.getParsedAccountInfo(programId),
  })

  /*
  const greet = useMutation({
    mutationKey: ['basic', 'greet', { cluster }],
    mutationFn: () => program.methods.greet().rpc(),
    onSuccess: (signature) => {
      transactionToast(signature)
    },
    onError: () => toast.error('Failed to run program'),
  })*/

  const initToken = useMutation({
    mutationKey: ['abl-token', 'init-token', { cluster }],
    mutationFn: (args: {
      decimals: number,
      mintAuthority: PublicKey,
      freezeAuthority: PublicKey,
      permanentDelegate: PublicKey,
      mode: string,
      threshold: BN,
      name: string,}
    ) => {
      const modeEnum = args.mode === 'allow' ? { allow: {} } : args.mode === 'block' ? { block: {}} : { mixed: {}}
      return program.methods.initMint({
        decimals: args.decimals,
        mintAuthority: args.mintAuthority,
        freezeAuthority: args.freezeAuthority,
        permanentDelegate: args.permanentDelegate,
        mode: modeEnum,
        threshold: args.threshold,
        name: args.name,
        symbol: '',
        uri: '',
      }).rpc()
    },
    onSuccess: (signature) => {
      transactionToast(signature)
    },
    onError: () => toast.error('Failed to run program'),
  })

  return {
    program,
    programId,
    getProgramAccount,
    initToken,
  }
}
