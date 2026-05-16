import Dexie, { type Table } from "dexie";
import type { RaesanTest } from "$lib/models";

export class Database extends Dexie {
	test_list!: Table<RaesanTest, string>;

	constructor() {
		super("raesan_tests");
		this.version(2)
			.stores({
				test_list: "id"
			})
			.upgrade(async (tx) => {
				await tx.table("test_list").clear();
			});
	}
}

export const db = new Database();
