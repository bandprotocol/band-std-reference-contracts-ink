import { DynamicModule, MiddlewareConsumer, Module } from '@nestjs/common'
import { ConfigModule, ConfigService } from '@nestjs/config'

import jsonConfig from '@shared/config/config'
import { DatabaseModule } from '@shared/database/database.module'
import { LoggerMiddleware } from '@shared/logger.middleware'

import { SubstrateModule } from '@substrate/substrate.module'

@Module({
  imports: [
    ConfigModule.forRoot({
      isGlobal: true,
      load: [jsonConfig],
      envFilePath: ['.env'],
    }),
  ],
})
export class AppModule {
  configure(consumer: MiddlewareConsumer) {
    consumer.apply(LoggerMiddleware).forRoutes('*')
  }

  static async register(configService: ConfigService): Promise<DynamicModule> {
    // Dynamically import chain module based on a config
    const modules: any[] = [DatabaseModule]
    const chainType = configService.getOrThrow('chain.chainType')
    switch (chainType) {
      case 'substrate':
        modules.push(SubstrateModule)
        break
      default:
        throw new Error(`Unknown chain type: ${chainType}`)
    }

    return {
      module: AppModule,
      imports: modules,
    }
  }
}
