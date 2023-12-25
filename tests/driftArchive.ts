import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DriftArchive } from "../target/types/drift_archive";

import {
  BN,
  AdminClient,
} from '@drift-labs/sdk';

import { Keypair } from '@solana/web3.js';
import {initializeQuoteSpotMarket, mockUSDCMint, mockUserUSDCAccount} from "./testHelpers";

describe("drift-archive", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local(undefined, {
    preflightCommitment: 'confirmed',
    skipPreflight: false,
    commitment: 'confirmed',
  });

  anchor.setProvider(provider);

  const adminClient = new AdminClient({
    connection: provider.connection,
    wallet: provider.wallet,
  });

  const program = anchor.workspace.DriftArchive as Program<DriftArchive>;

  let usdcMint: Keypair;
  let userUSDCAccount: Keypair;
  const usdcAmount = new BN(1000 * 10 ** 6);

  before(async () => {
    usdcMint = await mockUSDCMint(provider);
    userUSDCAccount = await mockUserUSDCAccount(usdcMint, usdcAmount, provider);
    await adminClient.initialize(usdcMint.publicKey, false);
    await adminClient.subscribe();
    await adminClient.initializeUserAccount();
    await initializeQuoteSpotMarket(adminClient, usdcMint.publicKey);
  });

  after(async () => {
    await adminClient.unsubscribe();
  });
});
