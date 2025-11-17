import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { PublicKey } from '@solana/web3.js';
import { useEffect, useState } from 'react';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { fetchCandyMachine, mintV2 } from '@metaplex-foundation/mpl-candy-machine';

// Substitua pelo ID real da sua Candy Machine
const CANDY_MACHINE_ID = new PublicKey(process.env.NEXT_PUBLIC_CANDY_MACHINE_ID || 'CndyV3LdqHUfS3ox1V6s9bN5aB2s3fGgH4A1bC2dE3F');

export const Minting = () => {
  const { connection } = useConnection();
  const { publicKey, signAllTransactions, signTransaction } = useWallet();
  const [candyMachineState, setCandyMachineState] = useState(null);
  const [isMinting, setIsMinting] = useState(false);
  const [message, setMessage] = useState('');

  useEffect(() => {
    if (!connection) return;

    const fetchState = async () => {
      try {
        // O Umi é a nova biblioteca do Metaplex para interagir com a Candy Machine
        const umi = createUmi(connection.rpcEndpoint);
        const cm = await fetchCandyMachine(umi, CANDY_MACHINE_ID);
        setCandyMachineState(cm);
      } catch (error) {
        console.error('Error fetching Candy Machine state:', error);
        setMessage('Erro ao carregar o estado da Candy Machine.');
      }
    };

    fetchState();
  }, [connection]);

  const handleMint = async () => {
    if (!publicKey || !signAllTransactions || !signTransaction || !candyMachineState) {
      setMessage('Por favor, conecte sua carteira.');
      return;
    }

    setIsMinting(true);
    setMessage('Minting em progresso...');

    try {
      const umi = createUmi(connection.rpcEndpoint);

      // A lógica de mintV2 é complexa e requer a configuração correta do Umi e do signer.
      // Este é um placeholder que demonstra a intenção.

      // Exemplo de chamada de mint (requer configuração completa do Umi com o wallet adapter)
      // const transaction = await mintV2(umi, {
      //   candyMachine: candyMachineState.publicKey,
      //   minter: publicKey,
      //   // ... outras contas necessárias
      // }).sendAndConfirm(umi);

      // Placeholder para simular o processo
      await new Promise(resolve => setTimeout(resolve, 2000));

      setMessage('NFT mintado com sucesso! Verifique sua carteira.');
    } catch (error) {
      console.error('Minting failed:', error);
      setMessage(`Falha no Minting: ${error.message}`);
    } finally {
      setIsMinting(false);
    }
  };

  const itemsRemaining = candyMachineState ? Number(candyMachineState.itemsRemaining) : '...';
  const price = '1.5 SOL'; // Hardcoded para o exemplo

  return (
    <div style={{ padding: '20px', textAlign: 'center' }}>
      <h1>Solana Speedsters Mint</h1>
      <WalletMultiButton />

      {publicKey && (
        <div style={{ marginTop: '20px' }}>
          <p>Itens Restantes: {itemsRemaining}</p>
          <p>Preço: {price}</p>

          <button 
            onClick={handleMint} 
            disabled={isMinting || itemsRemaining === 0}
            style={{ padding: '10px 20px', fontSize: '16px', cursor: 'pointer' }}
          >
            {isMinting ? 'Mintando...' : itemsRemaining === 0 ? 'Esgotado' : 'Mintar Carro NFT'}
          </button>

          <p style={{ marginTop: '10px', color: isMinting ? 'orange' : 'green' }}>{message}</p>
        </div>
      )}

      {!publicKey && <p style={{ marginTop: '20px' }}>Conecte sua carteira para começar.</p>}
    </div>
  );
};

export default Minting;
