import {
  Server,
  Account,
  StrKey,
  Networks,
} from '@stellar/stellar-sdk';

/**
 * Fetches account details from the Horizon server
 * @param publicKey The public key of the account
 * @param server The Stellar Horizon server instance
 * @returns The account object with details from the server
 * @throws Error if the account is not found or request fails
 */
export async function getAccount(
  publicKey: string,
  server: Server
): Promise<Account> {
  try {
    const account = await server.loadAccount(publicKey);
    return account;
  } catch (error) {
    throw new Error(
      `Failed to fetch account details for ${publicKey}: ${
        error instanceof Error ? error.message : 'Unknown error'
      }`
    );
  }
}

/**
 * Retrieves the native XLM balance for an account
 * @param publicKey The public key of the account
 * @param server The Stellar Horizon server instance
 * @returns The XLM balance as a string
 * @throws Error if account is not found or balance cannot be retrieved
 */
export async function getXlmBalance(
  publicKey: string,
  server: Server
): Promise<string> {
  try {
    const account = await server.loadAccount(publicKey);
    const xlmBalance = account.balances.find(
      (balance) => balance.asset_type === 'native'
    );

    if (!xlmBalance) {
      throw new Error('Native XLM balance not found for account');
    }

    return xlmBalance.balance;
  } catch (error) {
    throw new Error(
      `Failed to retrieve XLM balance for ${publicKey}: ${
        error instanceof Error ? error.message : 'Unknown error'
      }`
    );
  }
}

/**
 * Validates if a string is a valid Stellar public key
 * @param address The address string to validate
 * @returns True if the address is a valid Stellar public key, false otherwise
 */
export function isValidAddress(address: string): boolean {
  try {
    return StrKey.isValidEd25519PublicKey(address);
  } catch {
    return false;
  }
}

/**
 * Shortens a Stellar address for UI display
 * @param address The full address to truncate
 * @param chars Optional number of characters to show at start and end (default: 6)
 * @returns Shortened address in format GABCD...WXYZ
 * @throws Error if address is too short to truncate with given char count
 */
export function truncateAddress(address: string, chars: number = 6): string {
  if (address.length <= chars * 2 + 3) {
    return address;
  }

  const start = address.substring(0, chars);
  const end = address.substring(address.length - chars);
  return `${start}...${end}`;
}

/**
 * Returns the correct Stellar network passphrase
 * @param network The network type: 'TESTNET' or 'PUBLIC'
 * @returns The network passphrase string
 */
export function getNetworkPassphrase(
  network: 'TESTNET' | 'PUBLIC'
): string {
  if (network === 'TESTNET') {
    return Networks.TESTNET_NETWORK_PASSPHRASE;
  } else if (network === 'PUBLIC') {
    return Networks.PUBLIC_NETWORK_PASSPHRASE;
  }

  throw new Error(`Invalid network: ${network}`);
}
