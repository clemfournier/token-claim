import './App.css';
import {useEffect, useState} from 'react'
import * as anchor from "@project-serum/anchor";
import {Buffer} from 'buffer';
import idl from './idl.json' //get the smartcontract data structure model from target folder in anchor rust
import { Connection, PublicKey, clusterApiUrl, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { Program, AnchorProvider, web3, utils } from '@project-serum/anchor';
import { FeedPostDesign } from './FeedPostDesign';
import * as splToken from "@solana/spl-token";


window.Buffer = Buffer
const programID = new PublicKey(idl.metadata.address)
const network = clusterApiUrl("devnet")
const opts = {
  preflightCommitment:"processed",
}


const App = () => {
  const [Loading, setLoading] = useState(false)
  const [datas,setData] = useState([])
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
      if (solana) {
        if (solana.isPhantom) {
          const response = await solana.connect({
            onlyIfTrusted: true, //second time if anyone connected it won't show anypop on screen
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

  const initClaim = async() => {
    try {
      const provider = getProvider();
      const program = new Program(idl,programID,provider);
      const signer = new PublicKey(walletaddress);
      let escrow;
      [escrow] = await anchor.web3.PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode("claim"),
        signer.toBuffer()
      ], 
      program.programId);
      console.log('claimAccount', escrow.toString(), 'signer', signer.toString());
      const limit = new anchor.BN(10);
      const tx = await program.methods.initContract(limit)
      .accounts({
        claimAccount: escrow,
        signer,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .rpc({skipPreflight: true});

      console.log("TxSig :: ", tx);
    } catch (error) {
      console.log(error.message);
    }
  }

  const initContract = async() => {
    try {
      const provider = getProvider();
      const program = new Program(idl,programID,provider);
      const signer = new PublicKey(walletaddress);
      let escrow;
      [escrow] = await anchor.web3.PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode("claimcontract"),
        signer.toBuffer()
      ], 
      program.programId);
      console.log('claimContractAccount', escrow.toString(), 'signer', signer.toString());
      const limit = new anchor.BN(10);
      const tx = await program.methods.initContract(limit)
      .accounts({
        claimContractAccount: escrow,
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
      const signer = new PublicKey(walletaddress);
      const depositor = new PublicKey('EjvRc5HRynCfZu74QUDMs5iunHcKiSsyuKUxuNdgMFzz')

      let treasury;
      [treasury] = await anchor.web3.PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode("treasury6"),
        depositor.toBuffer()
      ], 
      program.programId);

      let escrow;
      [escrow] = await anchor.web3.PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode("claim"),
        signer.toBuffer()
      ], 
      program.programId);

      const tx = await program.methods.initClaim()
      .accounts({
        signer,
        depositor,
        claimAccount: escrow,
        treasury, //: new PublicKey('BqZUhaHrdBxyX8Rkqva5cmQb8nuoZztaRzpRmDZFpNt5'),
        treasuryTokenAccount: new PublicKey('6NDDmYTC4fwJzh17Bg2dRC3WkbN2fEySc1Rkr3CLKD1F'),
        claimerTokenAccount: new PublicKey('6Mac2LbWjvaUJXbHZ1w3Ux7mVYUDt74vsBXVvF21wwuB'),
        claimContractAccount: new PublicKey('DygGxBaqRi5G8ZfUo8SF6CjBdjU6wvpKqJP93vcmWTXq'),
        mint: new PublicKey('FUXjAEefwYaaoBAMq2Nx4wb4TwYxNmKox8JNWHKuwjWv'),
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
      let mint = new PublicKey('CCoin6VDphET1YsAgTGsXwThEUWetGNo4WiTPhGgR6US');
      let depositor_token_account = new PublicKey('6Sta9fu8asbk2qoGj3PXeVLxXTeJD6UvJb6WGkcbV1Kz');
      let treasuryTokenAccount = anchor.web3.Keypair.generate();

      console.log('treasury token account', treasuryTokenAccount.publicKey.toString())

      let escrow;
      [escrow] = await anchor.web3.PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode("treasury6"),
        depositor.toBuffer()
      ], 
      program.programId);
      const amount = new anchor.BN(10);
      
      const tx = await program.methods.initTreasury(amount)
        .accounts({
          depositor,
          mint,
          depositorTokenAccount: depositor_token_account,
          treasury: escrow,
          treasuryTokenAccount: treasuryTokenAccount.publicKey,
          tokenProgram: splToken.TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: anchor.web3.SystemProgram.programId
        })
        .signers([treasuryTokenAccount])
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
    </div>
  );
};

export default App;