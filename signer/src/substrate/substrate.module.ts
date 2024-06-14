import { Module } from '@nestjs/common'

import { DatabaseModule } from '@shared/database/database.module'
import { WalletModule } from '@shared/wallet/wallet.module'

import { SubstrateConfigService } from './config/substrate.config'
import { SubstrateController } from './substrate.controller'
import { SubstrateService } from './substrate.service'

@Module({
  controllers: [SubstrateController],
  providers: [SubstrateConfigService, SubstrateService],
  imports: [DatabaseModule, WalletModule],
})
export class SubstrateModule {}
