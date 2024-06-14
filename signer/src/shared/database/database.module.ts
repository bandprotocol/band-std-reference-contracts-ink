import { Module, Req } from '@nestjs/common'

import { PrismaService } from './prisma.service'
import { RequestService } from './request.service'

@Module({
  providers: [PrismaService, RequestService],
  exports: [RequestService],
})
export class DatabaseModule { }
