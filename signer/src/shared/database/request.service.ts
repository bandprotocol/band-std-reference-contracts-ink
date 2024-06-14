import { Injectable } from '@nestjs/common'

import { PrismaService } from './prisma.service'

@Injectable()
export class RequestService {
  constructor(private prisma: PrismaService) {}

  async setRequestRecord(network: string, payload: object): Promise<number> {
    const result = await this.prisma.request.create({
      data: {
        network: network,
        payload: JSON.stringify(payload),
      },
    })

    return result.id
  }

  async setRequestRecordSuccess(id: number, response: object): Promise<void> {
    await this.prisma.request.update({
      where: {
        id: id,
      },
      data: {
        success: true,
        response: JSON.stringify(response),
      },
    })
  }

  async setRequestRecordError(id: number, error: string): Promise<void> {
    await this.prisma.request.update({
      where: {
        id: id,
      },
      data: {
        success: false,
        error: error,
      },
    })
  }
}
