import { create } from 'zustand';

type Network = 'TESTNET' | 'PUBLIC';

interface WalletState {
  publicKey: string | null;
  connected: boolean;
  network: Network;
}

interface WalletActions {
  connect: (publicKey: string) => void;
  disconnect: () => void;
  setNetwork: (network: Network) => void;
}

type WalletStore = WalletState & WalletActions;

export const useWalletStore = create<WalletStore>((set) => ({
  publicKey: null,
  connected: false,
  network: 'TESTNET',

  connect: (publicKey) => set({ publicKey, connected: true }),

  disconnect: () => set({ publicKey: null, connected: false }),

  setNetwork: (network) => set({ network }),
}));
