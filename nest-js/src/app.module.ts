import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { DatabaseModule } from './database/_database.module';
import { RabbitMQModule } from './rabbitmq/_rabbitmq.module';

@Module({
  imports: [DatabaseModule, RabbitMQModule],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
