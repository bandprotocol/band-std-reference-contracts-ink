import { Module } from '@nestjs/common'

import { SubstrateWalletService } from './substrate/wallet.service'

@Module({
  providers: [SubstrateWalletService],
  exports: [SubstrateWalletService],
})
export class WalletModule {}
