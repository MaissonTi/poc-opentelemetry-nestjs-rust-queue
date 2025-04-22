import { AppService } from '@/app.service';
import { Controller } from '@nestjs/common';
import { Ctx, EventPattern, Payload, RmqContext } from '@nestjs/microservices';
import { context, propagation, trace } from '@opentelemetry/api';

@Controller()
export class ConsumerController {
  constructor(private readonly appService: AppService) {}

  @EventPattern('event_rust')
  async handleRustMessage(
    @Payload() payload: any,
    @Ctx() rmqContext: RmqContext,
  ) {
    const rawMessage = rmqContext.getMessage();

    const fullPayload = rawMessage.content.toString();

    const parsed = JSON.parse(fullPayload);

    const raw = parsed.traceContext ?? {};
    const carrier: Record<string, string> = {};

    for (const [k, v] of Object.entries(raw)) {
      if (typeof v === 'string') {
        carrier[k] = v;
      }
    }

    const extractedCtx = propagation.extract(context.active(), carrier);

    return context.with(extractedCtx, async () => {
      const tracer = trace.getTracer('nestjs');

      const span = tracer.startSpan('handle_rust_msg', undefined, extractedCtx); // ✅ contexto correto aqui

      await context.with(trace.setSpan(extractedCtx, span), async () => {
        await this.appService.consumer();
      });

      span.end();
    });
  }
}
