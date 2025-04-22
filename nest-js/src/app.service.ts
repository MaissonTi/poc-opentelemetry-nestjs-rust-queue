import { Injectable } from '@nestjs/common';
import { context, propagation } from '@opentelemetry/api';
import { randomUUID } from 'crypto';
import { UserRepository } from './database/prisma/repositories/user.repositories';
import { PublisherService } from './rabbitmq/publisher.service';

@Injectable()
export class AppService {
  constructor(
    private readonly userRepository: UserRepository,
    private readonly publisherService: PublisherService,
  ) {}

  private async createUser(context): Promise<any> {
    const data = {
      name: `test_${context}`,
      email: `test_${context}_${randomUUID()}@gmail.com`,
    };

    const value = await this.userRepository.create(data);
    return value;
  }

  async publisher(): Promise<void> {
    const data = await this.createUser('publisher_nest');

    const carrier: Record<string, string> = {};
    propagation.inject(context.active(), carrier);

    void this.publisherService.sendMessage('event_nest', {
      user: data,
      traceContext: carrier,
    });
  }

  async consumer(): Promise<void> {
    await this.createUser('consumer_nest');
    console.log('👍 Fim');
  }
}
