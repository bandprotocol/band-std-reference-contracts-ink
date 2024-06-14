import { Test, TestingModule } from '@nestjs/testing'

import { SubstrateWalletService } from '@shared/wallet/substrate/wallet.service'

import { SubstrateConfigService } from './config/substrate.config'
import { SubstrateService } from './substrate.service'

describe('SubstrateService', () => {
  let service: SubstrateService

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [
        SubstrateService,
        {
          provide: SubstrateConfigService,
          useValue: {
            config: {
              rpcURL: 'ws://localhost:9944',
            },
          },
        },
        {
          provide: SubstrateWalletService,
          useValue: {
            getKeyPair: jest.fn().mockReturnValue({}),
          },
        },
      ],
    }).compile()

    service = module.get<SubstrateService>(SubstrateService)
  })

  it('should be defined', () => {
    expect(service).toBeDefined()
  })
})
