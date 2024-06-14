import { IsNumber, IsString, ValidateNested } from 'class-validator'

import { PriceDataPayload } from '@shared/price'

export class SignDto {
  @ValidateNested()
  priceData: PriceDataPayload

  @IsString()
  from: string

  @IsNumber()
  nonce: number

  @IsNumber()
  tip: number
}
