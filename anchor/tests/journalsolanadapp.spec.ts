import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { Keypair } from '@solana/web3.js'
import { Journalsolanadapp } from '../target/types/journalsolanadapp'

describe('journalsolanadapp', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet

  const program = anchor.workspace.Journalsolanadapp as Program<Journalsolanadapp>

  const journalsolanadappKeypair = Keypair.generate()

  it('Initialize Journalsolanadapp', async () => {
    await program.methods
      .initialize()
      .accounts({
        journalsolanadapp: journalsolanadappKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([journalsolanadappKeypair])
      .rpc()

    const currentCount = await program.account.journalsolanadapp.fetch(journalsolanadappKeypair.publicKey)

    expect(currentCount.count).toEqual(0)
  })

  it('Increment Journalsolanadapp', async () => {
    await program.methods.increment().accounts({ journalsolanadapp: journalsolanadappKeypair.publicKey }).rpc()

    const currentCount = await program.account.journalsolanadapp.fetch(journalsolanadappKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Increment Journalsolanadapp Again', async () => {
    await program.methods.increment().accounts({ journalsolanadapp: journalsolanadappKeypair.publicKey }).rpc()

    const currentCount = await program.account.journalsolanadapp.fetch(journalsolanadappKeypair.publicKey)

    expect(currentCount.count).toEqual(2)
  })

  it('Decrement Journalsolanadapp', async () => {
    await program.methods.decrement().accounts({ journalsolanadapp: journalsolanadappKeypair.publicKey }).rpc()

    const currentCount = await program.account.journalsolanadapp.fetch(journalsolanadappKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Set journalsolanadapp value', async () => {
    await program.methods.set(42).accounts({ journalsolanadapp: journalsolanadappKeypair.publicKey }).rpc()

    const currentCount = await program.account.journalsolanadapp.fetch(journalsolanadappKeypair.publicKey)

    expect(currentCount.count).toEqual(42)
  })

  it('Set close the journalsolanadapp account', async () => {
    await program.methods
      .close()
      .accounts({
        payer: payer.publicKey,
        journalsolanadapp: journalsolanadappKeypair.publicKey,
      })
      .rpc()

    // The account should no longer exist, returning null.
    const userAccount = await program.account.journalsolanadapp.fetchNullable(journalsolanadappKeypair.publicKey)
    expect(userAccount).toBeNull()
  })
})
