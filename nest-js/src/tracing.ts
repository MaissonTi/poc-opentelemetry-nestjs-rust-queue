'use strict';

import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-grpc';
import { NestInstrumentation } from '@opentelemetry/instrumentation-nestjs-core';
import { resourceFromAttributes } from '@opentelemetry/resources';
import { NodeSDK } from '@opentelemetry/sdk-node';

import { AmqplibInstrumentation } from '@opentelemetry/instrumentation-amqplib';
import { ExpressInstrumentation } from '@opentelemetry/instrumentation-express';
import { HttpInstrumentation } from '@opentelemetry/instrumentation-http';
import { PrismaInstrumentation } from '@prisma/instrumentation';
const traceExporter = new OTLPTraceExporter({
  url: 'http://localhost:4317',
});

const sdk = new NodeSDK({
  traceExporter,
  resource: resourceFromAttributes({
    ['service.name']: 'nest-tracing',
  }),
  instrumentations: [
    new NestInstrumentation(),
    new HttpInstrumentation(),
    new ExpressInstrumentation(),
    new AmqplibInstrumentation({
      publishHook: (span, _info) => {
        span.setAttribute('messaging.system', 'rabbitmq');
        span.setAttribute('messaging.operation', 'publish');
      },
      consumeHook: (span, _info) => {
        span.setAttribute('messaging.system', 'rabbitmq');
        span.setAttribute('messaging.operation', 'consume');
      },
    }),
    new PrismaInstrumentation(),
  ],
});

process.on('SIGTERM', () => {
  sdk
    .shutdown()
    .then(() => console.log('Tracing terminated'))
    .catch((error) => console.log('Error terminating tracing', error))
    .finally(() => process.exit(0));
});

sdk.start();
