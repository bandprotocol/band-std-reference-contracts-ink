import { IsArray, IsInt, IsNumberString, IsString } from 'class-validator'

export class Price {
  @IsString()
  symbol: string
  @IsNumberString()
  rate: string
}
export class PriceDataPayload {
  @IsArray()
  prices: Price[]
  @IsInt()
  resolveTime: number
  @IsInt()
  requestId: number
}
