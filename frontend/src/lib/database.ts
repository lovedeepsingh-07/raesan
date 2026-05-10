import Dexie, { type Table } from "dexie";
import type { RaesanTest } from "$lib/models";

export class Database extends Dexie {
	test_list!: Table<RaesanTest, string>;

	constructor() {
		super("raesan_tests");
		this.version(1).stores({
			test_list: "id"
		});
	}
}

export const db = new Database();
