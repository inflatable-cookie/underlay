import { describe, expect, it } from "vitest";
import {
	formatDefaultAction,
	formatDefaultResourceType,
	getActionVariant,
	getDefaultActionType,
} from "../../src/components/log-list/helpers";

describe("components/log-list/helpers", () => {
	it("infers default action types from labels", () => {
		expect(getDefaultActionType("CreateUser")).toBe("create");
		expect(getDefaultActionType("edit_profile")).toBe("update");
		expect(getDefaultActionType("remove_item")).toBe("delete");
		expect(getDefaultActionType("recover_account")).toBe("restore");
		expect(getDefaultActionType("upload_asset")).toBe("upload");
		expect(getDefaultActionType("login")).toBe("login");
		expect(getDefaultActionType("sign_out")).toBe("logout");
		expect(getDefaultActionType("role_changed")).toBe("security");
		expect(getDefaultActionType("noop")).toBe("other");
	});

	it("maps action types to badge variants and formats labels", () => {
		expect(getActionVariant("create" as any)).toBe("success");
		expect(getActionVariant("delete" as any)).toBe("danger");
		expect(getActionVariant("update" as any)).toBe("info");
		expect(getActionVariant("login" as any)).toBe("muted");
		expect(getActionVariant("security" as any)).toBe("warning");
		expect(getActionVariant("other" as any)).toBe("default");

		expect(formatDefaultAction("account_locked_out")).toBe("account locked out");
		expect(formatDefaultResourceType("blog_post")).toBe("blog post");
	});
});
