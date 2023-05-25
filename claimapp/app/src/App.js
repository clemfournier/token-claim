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

  const cancel = async() =>{ //cancel button
    try {
      const provider = getProvider()
      const program = new Program(idl,programID,provider)
      let seller = new PublicKey('EjvRc5HRynCfZu74QUDMs5iunHcKiSsyuKUxuNdgMFzz');
      let escrow;
      [escrow] = await anchor.web3.PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode("escrow6"),
        seller.toBuffer()
      ], 
      program.programId);

      const tx = await program.methods.cancel()
      .accounts({
        claimer: new PublicKey('GvnCyJqkHBeEzrbafRtZVjBv17uaHvxXKmKtRv5Y6AwS'),
        seller,
        escrow: escrow,
        escrowedXTokens: new PublicKey('5qUSjHjiaLJeJ7KrJA5NDB1igd4pMsqZ68Buw7KDsSfi'),
        claimerXToken: new PublicKey('7vWSysD7pJomzXUK42PNoEL4cbk2LsTk3XihT8PSVBED'),
        tokenProgram: splToken.TOKEN_PROGRAM_ID
      })
      .rpc({skipPreflight: true})

      console.log("TxSig :: ", tx);
    } catch (error) {
      console.log(error.message);
    }
  }

  const createPostFunction = async(text,hastag,position) =>{ //createPostFunction connects to the smartcontract via rpc and lib.json  to create post
    const provider = getProvider() //checks & verify the dapp it can able to connect solana network
    const program = new Program(idl,programID,provider) //program will communicate to solana network via rpc using lib.json as model
    const num = new anchor.BN(position); //to pass number into the smartcontract need to convert into binary
    try{
      let x_mint = new PublicKey('CCoin6VDphET1YsAgTGsXwThEUWetGNo4WiTPhGgR6US');
      let y_mint = new PublicKey('JDB2uz6SAPhnNsFaRMSC4s4EKLS8jBEGPueNr9z59ohw');
      let sellers_x_token = new PublicKey('6Sta9fu8asbk2qoGj3PXeVLxXTeJD6UvJb6WGkcbV1Kz');
      let escrowedXTokens = anchor.web3.Keypair.generate();
      console.log("escrowedXTokens :: ", escrowedXTokens.publicKey.toString());
      let seller = new PublicKey(walletaddress);
      let escrow;
      [escrow] = await anchor.web3.PublicKey.findProgramAddress([
        anchor.utils.bytes.utf8.encode("escrow6"),
        seller.toBuffer()
      ], 
      program.programId);

      console.log(escrow.toString());
      
      const x_amount = new anchor.BN(40);
      const y_amount = new anchor.BN(10);
      
      const tx = await program.methods.initialize(x_amount, y_amount)
        .accounts({
          seller: seller,
          xMint: x_mint,
          yMint: y_mint,
          sellerXToken: sellers_x_token,
          escrow: escrow,
          escrowedXTokens: escrowedXTokens.publicKey,
          tokenProgram: splToken.TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: anchor.web3.SystemProgram.programId
        })
        .signers([escrowedXTokens])
        .rpc({skipPreflight: true})
  
      console.log("TxSig :: ", tx);
    }catch(err){
      console.log(err)
    }
  }

  // const getPosts = async() =>{
  //   const provider = getProvider();
  //   const program = new Program(idl,programID,provider)
  //   try{
  //     setLoading(true)
  //     Promise.all(
  //       ((await connection.getProgramAccounts(programID)).map(async(tx,index)=>( //no need to write smartcontract to get the data, just pulling all transaction respective programID and showing to user
  //         {
  //         ...(await program.account.feedPostApp.fetch(tx.pubkey)),
  //           pubkey:tx.pubkey.toString(),
  //       }
  //       )))
  //   ).then(result=>{
  //     result.sort(function(a,b){return b.position.words[0] - a.position.words[0] })
  //     setData([...result])
  //   })
  //   }catch(err){
  //     console.log(err)
  //   }finally{
  //     setLoading(false)
  //   }
  // }

  return (
    <div className='App'>
      <FeedPostDesign posts={datas} createPostFunction={createPostFunction}  walletaddress={walletaddress} connect={connect} Loading={Loading} />
      <a onClick={() => cancel()}>CANCEL</a>
    </div>
  );
};

export default App;