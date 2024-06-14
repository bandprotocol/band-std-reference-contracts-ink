import { BadRequestException, Body, Controller, HttpCode, Post, UseInterceptors } from '@nestjs/common'

import { LoggerDBInterceptor } from '@shared/interceptors/loggerdb.interceptor'
import { PriceDataPayload } from '@shared/price'

import { SignDto } from '@substrate/sign.dto'
import { SubstrateService } from '@substrate/substrate.service'

@UseInterceptors(LoggerDBInterceptor)
@Controller()
export class SubstrateController {
  constructor(
    private substrateService: SubstrateService,
  ) { }

  @Post('/*')
  @HttpCode(200)
  async index(@Body() payload: SignDto) {
    if (payload.priceData.prices.length === 0) {
      throw new BadRequestException('No symbol to sign')
    }

    // Sign by substrate service
    const prices = new PriceDataPayload()
    prices.prices = payload.priceData.prices
    prices.requestId = payload.priceData.requestId
    prices.resolveTime = payload.priceData.resolveTime

    const tx = await this.substrateService.sign(payload.from, prices, payload.nonce, payload.tip)
    return {
      tx: tx.toHex(),
    }
  }
}
