import { createClient } from "@supabase/supabase-js";

const init = <T>({
	url,
	authKey,
}: {
	url: string;
	authKey: string;
}) => createClient<T>(url, authKey);
export default init;
