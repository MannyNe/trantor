export function pluralized(amount: number, singular: string, plural: string) {
	return amount === 1 ? singular : plural;
}
