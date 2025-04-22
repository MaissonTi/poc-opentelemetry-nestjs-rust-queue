import { AppService } from '@/app.service';
import { DatabaseModule } from '@/database/_database.module';
import { Module } from '@nestjs/common';
import { ClientsModule, Transport } from '@nestjs/microservices';
import { ConsumerController } from './consumer.controller';
import { PublisherService } from './publisher.service';

@Module({
  imports: [
    ClientsModule.register([
      {
        name: 'RABBITMQ_SERVICE',
        transport: Transport.RMQ,
        options: {
          urls: ['amqp://guest:guest@localhost:5672'],
          queue: 'event_nest',
          queueOptions: { durable: false },
          exchange: 'exchange_direct',
        },
      },
    ]),
    DatabaseModule,
  ],
  providers: [PublisherService, AppService],
  controllers: [ConsumerController],
  exports: [PublisherService],
})
export class RabbitMQModule {}
