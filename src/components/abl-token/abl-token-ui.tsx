'use client'

import { PublicKey } from '@solana/web3.js'
import { useAblTokenProgram } from './abl-token-data-access'
import { Button } from '@/components/ui/button'
import { BN } from '@coral-xyz/anchor'

export function AblTokenCreate() {
  const { initToken } = useAblTokenProgram()

  return (
    <Button onClick={() => initToken.mutateAsync({
      decimals: 6,
      mintAuthority: new PublicKey(''),
      freezeAuthority: new PublicKey(''),
      permanentDelegate: new PublicKey(''),
      mode: 'allow',
      threshold: new BN(1),
      name: 'ablToken'
    })} disabled={initToken.isPending}>
      Run program{initToken.isPending && '...'}
    </Button>
  )
}

export function AblTokenProgram() {
  const { getProgramAccount } = useAblTokenProgram()

  if (getProgramAccount.isLoading) {
    return <span className="loading loading-spinner loading-lg"></span>
  }
  if (!getProgramAccount.data?.value) {
    return (
      <div className="alert alert-info flex justify-center">
        <span>Program account not found. Make sure you have deployed the program and are on the correct cluster.</span>
      </div>
    )
  }
  return (
    <div className={'space-y-6'}>
      <pre>{JSON.stringify(getProgramAccount.data.value, null, 2)}</pre>
    </div>
  )
}
