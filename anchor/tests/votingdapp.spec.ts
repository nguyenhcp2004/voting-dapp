import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { Voting } from '../target/types/voting'
import { PublicKey } from '@solana/web3.js'
import { startAnchor } from 'solana-bankrun'
import { BankrunProvider } from 'anchor-bankrun'

const IDL = require('../target/idl/voting.json')

const votingAddress = new PublicKey(
  'coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF'
)

jest.useRealTimers()

describe('Voting', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env())

  const votingProgram = anchor.workspace.Voting as Program<Voting>
  it('initializePoll', async () => {
    await votingProgram.methods
      .initializePoll(
        new anchor.BN(1),
        new anchor.BN(0),
        new anchor.BN(1834507850),
        'Who is the best?'
      )
      .rpc()

    const [pollAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, 'le', 8)],
      votingAddress
    )

    const poll = await votingProgram.account.pollAccount.fetch(pollAddress)

    console.log(poll)

    expect(poll.pollId.toNumber()).toBe(1)
    expect(poll.pollDescription).toBe('Who is the best?')
    expect(poll.pollVotingStart.toNumber()).toBeLessThan(
      poll.pollVotingEnd.toNumber()
    )
  })
})
