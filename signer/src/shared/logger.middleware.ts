import { Injectable, Logger, NestMiddleware } from '@nestjs/common'
import { NextFunction, Request, Response } from 'express'

@Injectable()
export class LoggerMiddleware implements NestMiddleware {
  private readonly logger = new Logger(LoggerMiddleware.name)

  use(req: Request, res: Response, next: NextFunction) {
    const { method, originalUrl, body } = req
    const timestamp = new Date().toISOString()

    // Log the request details using Logger
    this.logger.log(`[${timestamp}] ${method} ${originalUrl}`)
    this.logger.debug('Request Body:', body)

    // Continue with the request processing
    next()
  }
}
