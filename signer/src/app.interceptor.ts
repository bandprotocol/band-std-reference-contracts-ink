import { CallHandler, ExecutionContext, HttpException, Injectable, NestInterceptor } from '@nestjs/common'
import { Request } from 'express'
import { Observable, throwError } from 'rxjs'
import { catchError } from 'rxjs/operators'

@Injectable()
export class AppInterceptor implements NestInterceptor {
  async intercept(context: ExecutionContext, next: CallHandler): Promise<Observable<any>> {
    return next.handle().pipe(
      catchError((err) => {
        const request: Request = context.switchToHttp().getRequest()
        return throwError(
          () =>
            new HttpException(
              {
                message: err?.message || err?.detail || 'Internal server error',
                timestamp: new Date().toISOString(),
                route: request.path,
                method: request.method,
              },
              err.statusCode || 500,
            ),
        )
      }),
    )
  }
}
