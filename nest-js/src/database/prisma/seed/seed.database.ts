import { faker } from '@faker-js/faker';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

async function seed() {
  await prisma.user.deleteMany();

  const promise: Promise<any>[] = [];

  Array.from({ length: 30 }).map(async () => {
    promise.push(
      prisma.user.create({
        data: {
          name: faker.person.fullName(),
          email: faker.internet.email(),
        },
      }),
    );
  });

  await Promise.all(promise);
}

seed().then(() => {
  console.log('Seed completed');
  prisma.$disconnect();
});
