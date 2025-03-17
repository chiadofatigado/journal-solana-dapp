// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import JournalsolanadappIDL from '../target/idl/journalsolanadapp.json'
import type { Journalsolanadapp } from '../target/types/journalsolanadapp'

// Re-export the generated IDL and type
export { Journalsolanadapp, JournalsolanadappIDL }

// The programId is imported from the program IDL.
export const JOURNALSOLANADAPP_PROGRAM_ID = new PublicKey(JournalsolanadappIDL.address)

// This is a helper function to get the Journalsolanadapp Anchor program.
export function getJournalsolanadappProgram(provider: AnchorProvider, address?: PublicKey) {
  return new Program({ ...JournalsolanadappIDL, address: address ? address.toBase58() : JournalsolanadappIDL.address } as Journalsolanadapp, provider)
}

// This is a helper function to get the program ID for the Journalsolanadapp program depending on the cluster.
export function getJournalsolanadappProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the Journalsolanadapp program on devnet and testnet.
      return new PublicKey('coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF')
    case 'mainnet-beta':
    default:
      return JOURNALSOLANADAPP_PROGRAM_ID
  }
}
