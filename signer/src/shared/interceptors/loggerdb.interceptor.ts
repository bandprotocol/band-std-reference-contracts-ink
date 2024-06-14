import { CallHandler, ExecutionContext, Injectable, NestInterceptor } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { Observable } from 'rxjs'
import { tap } from 'rxjs/operators'

import { RequestService } from '@shared/database/request.service'

@Injectable()
export class LoggerDBInterceptor implements NestInterceptor {
  network: string

  constructor(
    private configService: ConfigService,
    private requestService: RequestService,
  ) { }

  onModuleInit() {
    this.network = this.configService.getOrThrow('chain.network')
  }

  async intercept(context: ExecutionContext, next: CallHandler): Promise<Observable<any>> {
    const ctx = context.switchToHttp()
    const rid = await this.requestService.setRequestRecord(this.network, ctx.getRequest().body)

    return next.handle().pipe(
      tap({
        next: (res) => {
          this.requestService.setRequestRecordSuccess(rid, res)
        },
        error: (err) => {
          this.requestService.setRequestRecordError(rid, err.message)
        },
      }),
    )
  }
}
