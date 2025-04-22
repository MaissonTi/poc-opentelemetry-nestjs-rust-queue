import { Injectable } from '@nestjs/common';
import { PrismaService } from '../prisma.service';

@Injectable()
export class UserRepository {
  constructor(protected readonly prisma: PrismaService) {}

  async create(data: any): Promise<any> {
    return this.prisma.user.create({
      data: {
        email: data.email,
        name: data.name,
      },
    });
  }
}
