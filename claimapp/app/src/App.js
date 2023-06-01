import './App.css';
import {useEffect, useState} from 'react'
import * as anchor from "@project-serum/anchor";
import {Buffer} from 'buffer';
import idl from './idl.json' //get the smartcontract data structure model from target folder in anchor rust
import { Connection, PublicKey, clusterApiUrl, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { Program, AnchorProvider } from '@project-serum/anchor';
import * as splToken from "@solana/spl-token";


window.Buffer = Buffer
const programID = new PublicKey(idl.metadata.address)
const network = clusterApiUrl("devnet")
const opts = {
  preflightCommitment:"processed",
}
const depositor = new PublicKey('EjvRc5HRynCfZu74QUDMs5iunHcKiSsyuKUxuNdgMFzz');
const claimContractAccount = new PublicKey('AooKy14uWdPQbQHSPd3oEZ4epK4ism6NYfxiBjYpPB7Y');
const mint = new PublicKey('CCoin6VDphET1YsAgTGsXwThEUWetGNo4WiTPhGgR6US');
const treasuryId = 'treasury8';
const contractId = 'contract8';
const claimId = 'claim8';

const App = () => {
  const [Loading, setLoading] = useState(false)
  const [walletaddress, setWalletAddress] = useState("");
  
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

  const onLoad = () => {
    checkIfWalletIsConnected();
    // getPosts();
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

  const connect = () => {
    return (
      <button onClick={connectWalletRenderPopup} className="buttonStyle"> {Loading ? <p>loading...</p>: <p>Connect Your Wallet To Post </p>}    </button>
    );
  };

  const initContract = async() => {
    try {
      const provider = getProvider();
      const program = new Program(idl,programID,provider);
      const signer = new PublicKey(walletaddress);
      // let escrow = anchor.web3.Keypair.generate();
      let contract;
      [contract] = await anchor.web3.PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode(contractId)
      ], 
      program.programId);
      console.log('claimContract', contract.toString(), 'signer', signer.toString());
      const limit = new anchor.BN(10);
      const tx = await program.methods.initContract(limit)
      .accounts({
        claimContract: contract,
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

      const tx = await program.methods.initClaim()
      .accounts({
        signer,
        claimAccount,
        treasury, //: new PublicKey('BqZUhaHrdBxyX8Rkqva5cmQb8nuoZztaRzpRmDZFpNt5'),
        treasuryTokenAccount: new PublicKey('jkmbmZKfdBdFwQit12tPTCchFBGmhHFx58ANsH919Hz'),
        claimerTokenAccount: new PublicKey('3gDPGmt2gtiqFY79ayYdtcvqQSHEXKYCWvPUQZ7ZyVfa'),
        claimContract,
        nftTokenAccount: new PublicKey('Czs8q51C3jfUNY9pXFwQ61Q7uc4H18gqFjQhrBvQeHC2'),
        nftMetadata: new PublicKey('sGHgm6DsCpG8WkhnEmNqzFDguq77xTERYr5QUtBJhpH'),
        // mint: new PublicKey('46pcSL5gmjBrPqGKFaLbbCmR6iVuLJbnQy13hAe7s6CC'),
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
      let depositor_token_account = new PublicKey('6Mac2LbWjvaUJXbHZ1w3Ux7mVYUDt74vsBXVvF21wwuB');
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
      // const amount = new anchor.BN(100000000);
      
      const tx = await program.methods.initTreasury(amount)
        .accounts({
          depositor,
          mint,
          depositorTokenAccount: depositor_token_account,
          treasury: escrow,
          solTreasury: solTreasury.publicKey,
          treasuryTokenAccount: treasuryTokenAccount.publicKey,
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
      let mint = new PublicKey('CCoin6VDphET1YsAgTGsXwThEUWetGNo4WiTPhGgR6US');
      let depositor_token_account = new PublicKey('6Mac2LbWjvaUJXbHZ1w3Ux7mVYUDt74vsBXVvF21wwuB');
      let treasuryTokenAccount = new PublicKey('jkmbmZKfdBdFwQit12tPTCchFBGmhHFx58ANsH919Hz');
      let treasury;
      [treasury] = await anchor.web3.PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode(treasuryId),
        // depositor.toBuffer()
      ], 
      program.programId);
      
      const amount = new anchor.BN(100000000);
      const sol_amount = new anchor.BN(100000000);
      
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
      <div>
        <a style={{color: 'white'}} onClick={() => initTreasury()}>INIT TREASURY</a>
      </div>
      <div>
        <a style={{color: 'white'}} onClick={() => initClaimV2()}>INIT CLAIM</a>
      </div>
      <div>
        <a style={{color: 'white'}} onClick={() => initContract()}>INIT CONTRACT</a>
      </div>
      <div>
        <a style={{color: 'white'}} onClick={() => addToTreasury()}>ADD TO TREASURY</a>
      </div>
    </div>
  );
};

export default App;