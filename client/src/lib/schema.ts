import z from 'zod';

const countSchema = z.object({
	count: z
		.number()
		.nullish()
		.transform((v) => v || 0)
});
const weekdayCountSchema = countSchema.extend({ weekday: z.number() });
const hourCountSchema = countSchema.extend({ hour: z.number() });
const osCountSchema = countSchema.extend({ os: z.string() });
const browserCountSchema = countSchema.extend({ browser: z.string() });
const deviceCountSchema = countSchema.extend({ device: z.string() });

export const TrackingDataSchema = z.object({
	name: z.string(),

	session_count_by_weekday: z.array(weekdayCountSchema),
	visitor_count_by_weekday: z.array(weekdayCountSchema),

	session_count_by_hour: z.array(hourCountSchema),
	visitor_count_by_hour: z.array(hourCountSchema),

	visitor_count_by_os: z.array(osCountSchema),
	visitor_count_by_browser: z.array(browserCountSchema),
	visitor_count_by_device: z.array(deviceCountSchema)
});

export const TrackingSchema = z.object({
	id: z.string(),
	name: z.string(),
	created_at: z.number().transform((v) => new Date(v)),

	visitor_count: z.number(),
	sessions_count: z.number(),
	events_count: z.number(),
	sources_count: z.number()
});

export const TrackingsSchema = z.object({
	trackings: z.array(TrackingSchema)
});
