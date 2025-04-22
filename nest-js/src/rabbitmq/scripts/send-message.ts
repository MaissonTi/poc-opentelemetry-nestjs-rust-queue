import { ClientProxyFactory, Transport } from '@nestjs/microservices';

async function bootstrap() {
  const client = ClientProxyFactory.create({
    transport: Transport.RMQ,
    options: {
      urls: ['amqp://guest:guest@localhost:5672'],
      queue: 'main_queue',
      queueOptions: { durable: false },
    },
  });

  const payload = {
    id: `${Math.floor(Math.random() * 1000)}z`,
    name: 'test User',
    email: 'test@example.com',
  };

  console.log('Sending message to RabbitMQ...');
  client.emit('user_created', payload).subscribe({
    complete: () => {
      console.log('Message sent');
      process.exit(0);
    },
    error: (err) => {
      console.error('Error sending message:', err);
      process.exit(1);
    },
  });
}

bootstrap();
