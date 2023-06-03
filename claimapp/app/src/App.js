import './App.css';
import {useEffect, useState} from 'react'
import * as anchor from "@project-serum/anchor";
import {Buffer} from 'buffer';
import idl from './idl.json' //get the smartcontract data structure model from target folder in anchor rust
import { Connection, PublicKey, clusterApiUrl, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { Program, AnchorProvider } from '@project-serum/anchor';
import * as splToken from "@solana/spl-token";
import { publicKey, u64, bool } from '@solana/buffer-layout-utils';
import { u32, u8, struct, blob } from '@solana/buffer-layout';

window.Buffer = Buffer
const programID = new PublicKey(idl.metadata.address)
const network = clusterApiUrl("devnet")
const opts = {
  preflightCommitment:"processed",
}
const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const mint = new PublicKey('CCoin6VDphET1YsAgTGsXwThEUWetGNo4WiTPhGgR6US');
const nftTokenAccount = new PublicKey('Czs8q51C3jfUNY9pXFwQ61Q7uc4H18gqFjQhrBvQeHC2');
const nftMetadata = new PublicKey('sGHgm6DsCpG8WkhnEmNqzFDguq77xTERYr5QUtBJhpH');
const treasuryId = 'treasury9';
const contractId = 'contract9';
const claimId = 'claim9';

const App = () => {
  const [Loading, setLoading] = useState(false)
  const [walletaddress, setWalletAddress] = useState("");
  const [contract, setContract] = useState(null);
  
  const { solana } = window;
  const getProvider = () => {
    //Creating a provider, the provider is authenication connection to solana
    const connection = new Connection(network, opts.preflightCommitment);
    const provider = new AnchorProvider(
      connection,
      window.solana,
      opts.preflightCommitment
    );
    return provider;
  };
  
  const getTokenAccountByAccountAndMint = async (accountAddress, mintAddress, connection) => {
    const associatedTokenAccounts = await connection.getParsedTokenAccountsByOwner(
      accountAddress,
      {
        mint: mintAddress,
        programId: TOKEN_PROGRAM_ID,
      }
    );
  
    // Extract the token account addresses from the result
    const tokenAccounts = associatedTokenAccounts.value.map((account) =>
      account.pubkey.toString()
    );

    if (tokenAccounts.length === 1) {
      return tokenAccounts[0];
    } else {
      return null;
    }
  }

  const checkIfWalletIsConnected = async () => {
    try {
      setLoading(true)
      const { solana } = window;
      console.log('solana', solana);

      if (solana) {
        console.log("solana object found", solana.isPhantom);
        if (solana.isPhantom) {
          const response = await solana.connect({
            onlyIfTrusted: false, //second time if anyone connected it won't show anypop on screen
          });
          setWalletAddress(response.publicKey.toString());
        }
      } else {
        alert("Solana object not found!, Get a Phantom Wallet");
      }
    } catch (error) {
      console.log(error.message);
    }finally{
      setLoading(false)
    }
  };

  useEffect(() => {
    window.addEventListener("load", onLoad);
    return () => window.removeEventListener("load", onLoad);
  }, []);

  const fetchAndParseMint = async (mint, solanaConnection) => {
    try {
        // console.log(`Step - 1: Fetching Account Data for ${mint.toBase58()}`);
        let {data} = await solanaConnection.getAccountInfo(mint) || {};
        if (!data) return;
        // console.log(`Step - 2: Deserializing Found Account Data`);
        const deserialized = MintLayout.decode(data);
        console.log(deserialized);
        console.log(deserialized.mint.toString());
        console.log(deserialized.update_authority.toString());
        setContract(deserialized);
    }
    catch {
        return null;
    }
}

  const getContractData = async(claimContract) => {
    // Fetch contract data
    fetchAndParseMint(claimContract, getProvider().connection);
  }

  const loadData = async() => {
    const provider = getProvider();
    const program = new Program(idl,programID,provider);
    let claimContract;
    [claimContract] = await anchor.web3.PublicKey.findProgramAddress([
      anchor.utils.bytes.utf8.encode(contractId)
    ], 
    program.programId);
    console.log('claimContract', claimContract.toString());
    console.log('program.programId', program.programId.toString());
    getContractData(claimContract);
  }

  const onLoad = async () => {
    checkIfWalletIsConnected();
  };

  const connectWalletRenderPopup = async () => { //first time users are connecting to wallet this function will activate
    try{
      setLoading(true)
      if (solana) {
        const response = await solana.connect();
        setWalletAddress(response.publicKey.toString());
      }
    }catch(err){
      console.log(err)
    }finally{
      setLoading(false)
    }
  };

  const disconnectWalletRenderPopup = async () => { //first time users are connecting to wallet this function will activate
    setWalletAddress(null);
  };

  const initContract = async() => {
    try {
      const provider = getProvider();
      const program = new Program(idl,programID,provider);
      const signer = new PublicKey(walletaddress);
      // let escrow = anchor.web3.Keypair.generate();
      let contract;
      [contract] = anchor.web3.PublicKey.findProgramAddressSync([
        anchor.utils.bytes.utf8.encode(contractId)
      ], 
      program.programId);
      console.log('claimContract', contract.toString(), 'signer', signer.toString());
      const limit = new anchor.BN(10);
      const claimAmount = new anchor.BN(100000000);
      const tx = await program.methods.initContract(limit, 'VU5ERVJET0cAAA==', claimAmount)
      .accounts({
        claimContract: contract,
        mint: new PublicKey('CCoin6VDphET1YsAgTGsXwThEUWetGNo4WiTPhGgR6US'),
        updateAuthority: new PublicKey('En54STTsmVrWA3Cd43SQNgiLrihRDG2iMJD6zWPHjYfW'),
        signer,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .rpc({skipPreflight: true});

      console.log("TxSig :: ", tx);
    } catch (error) {
      console.log(error.message);
    }
  }

  const updateContract = async() => {
    try {
      const provider = getProvider();
      const program = new Program(idl,programID,provider);
      const signer = new PublicKey(walletaddress);
      // let escrow = anchor.web3.Keypair.generate();
      let contract;
      [contract] = anchor.web3.PublicKey.findProgramAddressSync([
        anchor.utils.bytes.utf8.encode(contractId)
      ], 
      program.programId);
      console.log('claimContract', contract.toString(), 'signer', signer.toString());
      const limit = new anchor.BN(10);
      const claimAmount = new anchor.BN(100000);
      const tx = await program.methods.updateContract(limit, 'VU5ERVJET0cAAA==', claimAmount, true)
      .accounts({
        claimContract: contract,
        updateAuthority: new PublicKey('En54STTsmVrWA3Cd43SQNgiLrihRDG2iMJD6zWPHjYfW'),
        signer,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .rpc({skipPreflight: true});

      console.log("TxSig :: ", tx);
    } catch (error) {
      console.log(error.message);
    }
  }

  const initClaimV2 = async() =>{ //cancel button
    try {
      const provider = getProvider();
      const program = new Program(idl,programID,provider);
      console.log('walletaddress', walletaddress);
      
      const signer = new PublicKey(walletaddress);

      let treasury;
      [treasury] = await anchor.web3.PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode(treasuryId)
      ], 
      program.programId);

      let claimContract;
      [claimContract] = await anchor.web3.PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode(contractId)
      ], 
      program.programId);

      let claimAccount;
      [claimAccount] = await anchor.web3.PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode(claimId),
        signer.toBuffer()
      ], 
      program.programId);

      const treasuryTokenAccount = await getTokenAccountByAccountAndMint(treasury, mint, provider.connection);
      const claimerTokenAccount = await getTokenAccountByAccountAndMint(new PublicKey(walletaddress), mint, provider.connection);

      const tx = await program.methods.initClaim(true)
      .accounts({
        signer,
        claimAccount,
        treasury,
        treasuryTokenAccount,
        claimerTokenAccount,
        claimContract,
        nftTokenAccount,
        nftMetadata,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .rpc({skipPreflight: true})

      console.log("TxSig :: ", tx);
    } catch (error) {
      console.log(error);
    }
  }

  const initTreasury = async() => {
    const provider = getProvider();
    const program = new Program(idl,programID,provider);

    try{
      let depositor = new PublicKey(walletaddress);
      const depositor_token_account = await getTokenAccountByAccountAndMint(depositor, mint, provider.connection);
      // let depositor_token_account = new PublicKey('6Mac2LbWjvaUJXbHZ1w3Ux7mVYUDt74vsBXVvF21wwuB');
      // let depositor_token_account = new PublicKey('3gDPGmt2gtiqFY79ayYdtcvqQSHEXKYCWvPUQZ7ZyVfa');
      let treasuryTokenAccount = anchor.web3.Keypair.generate();
      let solTreasury = anchor.web3.Keypair.generate();

      console.log('treasury token account', treasuryTokenAccount.publicKey.toString())

      let escrow;
      [escrow] = await anchor.web3.PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode(treasuryId),
        // depositor.toBuffer()
      ], 
      program.programId);
      const amount = new anchor.BN(1);
      let claimContract;
      [claimContract] = await anchor.web3.PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode(contractId),
        // depositor.toBuffer()
      ], 
      program.programId);
      // const amount = new anchor.BN(100000000);
      
      const tx = await program.methods.initTreasury(amount)
        .accounts({
          depositor,
          mint,
          depositorTokenAccount: depositor_token_account,
          treasury: escrow,
          solTreasury: solTreasury.publicKey,
          treasuryTokenAccount: treasuryTokenAccount.publicKey,
          claimContract,
          tokenProgram: splToken.TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: anchor.web3.SystemProgram.programId
        })
        .signers([treasuryTokenAccount, solTreasury])
        .rpc({skipPreflight: true})
  
      console.log("TxSig :: ", tx);
    }catch(err){
      console.log(err)
    }
  }

  const addToTreasury = async() => {
    const provider = getProvider();
    const program = new Program(idl,programID,provider);

    try{
      console.log('depositor' , walletaddress);
      let depositor = new PublicKey(walletaddress);
      let depositor_token_account = await getTokenAccountByAccountAndMint(depositor, mint, provider.connection);
      console.log('depositor_token_account', depositor_token_account.toString());
      let treasury;
      [treasury] = await anchor.web3.PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode(treasuryId),
        // depositor.toBuffer()
      ], 
      program.programId);
      let treasuryTokenAccount = await getTokenAccountByAccountAndMint(treasury, mint, provider.connection);
      
      const amount = new anchor.BN(1000000000);
      const sol_amount = new anchor.BN(1000000000);
      
      const tx = await program.methods.addToTreasury(amount, sol_amount)
        .accounts({
          depositor,
          mint,
          depositorTokenAccount: depositor_token_account,
          treasury,
          treasuryTokenAccount: treasuryTokenAccount,
          tokenProgram: splToken.TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: anchor.web3.SystemProgram.programId
        })
        .rpc({skipPreflight: true})
  
      console.log("TxSig :: ", tx);
    }catch(err){
      console.log(err)
    }
  }

  return (
    <div className='App'>
      <div style={{marginTop: '2px'}}>
        <a style={{color: 'white', cursor: 'pointer'}} onClick={() => initTreasury()}>INIT TREASURY</a>
      </div>
      <div style={{marginTop: '2px'}}>
        <a style={{color: 'white', cursor: 'pointer'}} onClick={() => initClaimV2()}>INIT CLAIM</a>
      </div>
      <div style={{marginTop: '2px'}}>
        <a style={{color: 'white', cursor: 'pointer'}} onClick={() => initContract()}>INIT CONTRACT</a>
      </div>
      <div style={{marginTop: '2px'}}>
        <a style={{color: 'white', cursor: 'pointer'}} onClick={() => addToTreasury()}>ADD TO TREASURY</a>
      </div>
      <div style={{marginTop: '2px'}}>
        <a style={{color: 'white', cursor: 'pointer'}} onClick={() => loadData()}>LOAD DATA</a>
      </div>
      <div style={{marginTop: '2px'}}>
        <a style={{color: 'white', cursor: 'pointer'}} onClick={() => updateContract()}>UPDATE CONTRACT</a>
      </div>
      <div style={{marginTop: '20px'}}>
        <a style={{color: 'white', cursor: 'pointer'}} onClick={() => connectWalletRenderPopup()}>{walletaddress ? walletaddress : 'CONNECT'}</a>
      </div>
      <div>
        <a style={{color: 'white', cursor: 'pointer'}} onClick={() => disconnectWalletRenderPopup()}>{walletaddress ? 'DISCONNECT' : ''}</a>
      </div>
      {
        contract != null ? (
          <div>
            <div style={{marginTop: '15px'}}>
              <span style={{color: 'white'}}>LIMIT: {Number(contract.limit)}</span>
            </div>
            <div style={{marginTop: '1px'}}>
              <span style={{color: 'white'}}>CLAIMED: {Number(contract.claimed)}</span>
            </div>
            <div style={{marginTop: '1px'}}>
              <span style={{color: 'white'}}>IS ACTIVE: {contract.isActive.toString()}</span>
            </div>
          </div>
        ) : ''
      }
    </div>
  );
};

export default App;

export const MintLayout = struct([
  u64('discriminator'),
  bool('isActive'),
  u64('claimed'),
  u64('limit'),
  u8('bump'),
  publicKey('mint'),
  publicKey('update_authority'),
  u64('claim_amount'),
  blob(32, 'collection_name')
]);