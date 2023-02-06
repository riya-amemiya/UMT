import { createClient } from "@supabase/supabase-js";

const database = async <T>({
	supabase,
	type,
}: {
	supabase: ReturnType<typeof createClient<T>>;
	type: string;
}) => {
	if (type === "fetch") {
		return async ({
			from,
		}: {
			from: string;
		}) => await supabase.from(from).select("*");
	}
	return null;
};
export default database;
