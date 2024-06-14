import { BadRequestException } from '@nestjs/common'
import { Test, TestingModule } from '@nestjs/testing'

import { SignDto } from '@substrate/sign.dto'
import { SubstrateService } from '@substrate/substrate.service'

import { SubstrateController } from './substrate.controller'

describe('SubstrateController', () => {
  let substrateController: SubstrateController
  let substrateService: SubstrateService

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      controllers: [SubstrateController],
      providers: [
        { provide: SubstrateService, useValue: { sign: jest.fn() } },
      ],
    }).compile()

    substrateController = module.get<SubstrateController>(SubstrateController)
    substrateService = module.get<SubstrateService>(SubstrateService)
  })

  it('should be defined', () => {
    expect(substrateController).toBeDefined()
  })

  it('should throw BadRequestException when there are no prices', async () => {
    const signDto: SignDto = {
      priceData: { prices: [{ symbol: 'symbol', rate: '1' }], requestId: 1, resolveTime: 1705393641 },
      from: 'address',
      nonce: 1,
      tip: 0,
    }

    await expect(substrateController.index(signDto)).rejects.toThrow(BadRequestException)
  })

  it('should return tx', async () => {
    const signDto: SignDto = {
      priceData: { prices: [{ symbol: 'symbol', rate: '1' }], requestId: 1, resolveTime: 1705393641 },
      from: 'address',
      nonce: 1,
      tip: 0,
    }

    jest.spyOn(substrateService, 'sign').mockResolvedValue({ toHex: () => '0x00' } as any)
    const result = await substrateController.index(signDto)

    expect(substrateService.sign).toHaveBeenCalled()
    expect(result).toEqual({ tx: '0x00' })
  })
})
