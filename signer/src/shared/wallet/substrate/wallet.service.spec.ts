import { ConfigService } from '@nestjs/config'
import { Test, TestingModule } from '@nestjs/testing'

import { SubstrateWalletService } from './wallet.service'

// Mocking the ConfigService
const mockConfigService = {
  getOrThrow: jest.fn(),
}

describe('SubstrateWalletService', () => {
  let service: SubstrateWalletService

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [SubstrateWalletService, { provide: ConfigService, useValue: mockConfigService }],
    }).compile()

    service = module.get<SubstrateWalletService>(SubstrateWalletService)
  })

  it('should be defined', () => {
    expect(service).toBeDefined()
  })
})
