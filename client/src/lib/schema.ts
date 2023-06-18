import z from 'zod';

const countSchema = z.object({
	count: z.number()
});
const weekdayCountSchema = countSchema.extend({ weekday: z.number() });
const hourCountSchema = countSchema.extend({ hour: z.number() });
const osCountSchema = countSchema.extend({ os: z.string() });
const browserCountSchema = countSchema.extend({ browser: z.string() });
const deviceCountSchema = countSchema.extend({ device: z.string() });
const pathCountSchema = countSchema.extend({ pathname: z.string() });
const titleCountSchema = countSchema.extend({ title: z.string() });
const countryCountSchema = countSchema.extend({
	name: z
		.string()
		.nullable()
		.transform((v) => v ?? 'Unknown'),
	iso_code: z
		.string()
		.nullable()
		.transform((v) => v ?? 'Unknown')
});
const referralCountSchema = countSchema.extend({
	referral: z
		.string()
		.nullable()
		.transform((v) => v ?? 'Unknown')
});

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

const TrackingSchema = z.object({
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

const VisitorAndSessionCount = z.object({
	visitor_count: z.number(),
	session_count: z.number()
});

const SourceSchema = VisitorAndSessionCount.extend({ name: z.string() });
const RefererSchema = VisitorAndSessionCount.extend({ referer: z.string() });

const countSort = <T extends { count: number }>(list: T[]) =>
	list.sort((a, b) => b.count - a.count);
const sessionSort = <T extends { session_count: number }>(list: T[]) =>
	list.sort((a, b) => b.session_count - a.session_count);

export const TrackingCounts = z.object({
	sources: z.array(SourceSchema).transform(sessionSort),
	paths: z.array(pathCountSchema).transform(countSort),
	titles: z.array(titleCountSchema).transform(countSort),
	refers: z.array(RefererSchema).transform(sessionSort),
	countries: z.array(countryCountSchema).transform(countSort),
	referrals: z.array(referralCountSchema).transform(countSort)
});
