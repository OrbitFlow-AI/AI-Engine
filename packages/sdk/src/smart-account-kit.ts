// Smart Account Kit integration hooks for passkey-based agent wallets.
import type { SmartAccountConfig } from './types.js';

export interface SignedTransaction {
  txHash: string;
  result?: number | string;
}

export interface TransactionPayload {
  contractId: string;
  method: string;
  args: Record<string, unknown>;
  [key: string]: unknown;
}

export class SmartAccountKitAdapter {
  private readonly config: SmartAccountConfig;
  private sessionKeyExpiry: number = 0;

  constructor(config: SmartAccountConfig) {
    this.config = config;
  }

  /** Create a new agent smart account with passkey credentials. */
  async createAgentWallet(agentId: string): Promise<string> {
    const response = await this.callKit('/accounts/create', {
      agentId,
      passkey: this.config.passkeyCredential ? 'present' : 'generate',
    });

    this.sessionKeyExpiry = Date.now() + this.config.sessionKeyTtlSeconds * 1000;
    return response.address as string;
  }

  /** Sign a Soroban contract invocation via delegated session key. */
  async signTransaction(payload: TransactionPayload): Promise<SignedTransaction> {
    if (Date.now() > this.sessionKeyExpiry) {
      throw new Error('Session key expired. Call rotateSessionKey().');
    }

    const response = await this.callKit('/transactions/sign', payload);
    return {
      txHash: response.txHash as string,
      result: response.result as number,
    };
  }

  /** Rotate delegated session key for continued agent autonomy. */
  async rotateSessionKey(agentAddress: string): Promise<void> {
    await this.callKit('/accounts/rotate-key', { address: agentAddress });
    this.sessionKeyExpiry = Date.now() + this.config.sessionKeyTtlSeconds * 1000;
  }

  private async callKit(
    path: string,
    body: Record<string, unknown>,
  ): Promise<Record<string, unknown>> {
    const url = `${this.config.kitUrl}${path}`;

    // Stub response for scaffold — real HTTP call in production
    if (path === '/accounts/create') {
      return { address: `G${(body.agentId as string).slice(0, 54).padEnd(54, 'X')}` };
    }
    if (path === '/transactions/sign') {
      return { txHash: `tx_${Date.now()}`, result: 1 };
    }
    return {};
  }
}
