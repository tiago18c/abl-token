'use client'

import { PublicKey } from '@solana/web3.js'
import { useAblTokenProgram } from './abl-token-data-access'
import { Button } from '@/components/ui/button'
import { BN } from '@coral-xyz/anchor'
import React from 'react'

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

interface WalletEntry {
  address: string;
  mode: 'allow' | 'block';
}

export function AblTokenWalletTable() {
  const [wallets, setWallets] = React.useState<WalletEntry[]>([]);

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
  };

  const handleDrop = async (e: React.DragEvent) => {
    e.preventDefault();

    const file = e.dataTransfer.files[0];
    if (file && file.type === 'text/csv') {
      const text = await file.text();
      const rows = text.split('\n');
      
      const parsed: WalletEntry[] = rows
        .filter(row => row.trim()) // Skip empty rows
        .map(row => {
          const [address, mode] = row.split(',').map(field => field.trim());
          return {
            address,
            mode: mode.toLowerCase() as 'allow' | 'block'
          };
        })
        .filter(entry => {
          // Basic validation
          try {
            new PublicKey(entry.address);
            return ['allow', 'block'].includes(entry.mode);
          } catch {
            return false;
          }
        });

      setWallets(parsed);
    }
  };

  return (
    <div className="space-y-4">
      <div 
        onDragOver={handleDragOver}
        onDrop={handleDrop}
        className="border-2 border-dashed rounded-lg p-8 text-center hover:border-primary cursor-pointer"
      >
        Drop CSV file here (address,mode)
      </div>

      {wallets.length > 0 && (
        <div className="overflow-x-auto">
          <table className="table w-full">
            <thead>
              <tr>
                <th>Address</th>
                <th>Mode</th>
              </tr>
            </thead>
            <tbody>
              {wallets.map((wallet, index) => (
                <tr key={index}>
                  <td className="font-mono">{wallet.address}</td>
                  <td>
                    <span className={`badge ${wallet.mode === 'allow' ? 'badge-success' : 'badge-error'}`}>
                      {wallet.mode}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
