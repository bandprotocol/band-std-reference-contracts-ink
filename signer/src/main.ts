import { ValidationPipe } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { NestFactory } from '@nestjs/core'

import { AppInterceptor } from './app.interceptor'
import { AppModule } from './app.module'

async function bootstrap() {
  const appContext = await NestFactory.createApplicationContext(AppModule)
  const configService = appContext.get(ConfigService)

  const dynamicModule = await AppModule.register(configService)

  const app = await NestFactory.create(dynamicModule)
  app.useGlobalPipes(new ValidationPipe())
  app.useGlobalInterceptors(new AppInterceptor())

  await app.listen(8080)
}
bootstrap()
