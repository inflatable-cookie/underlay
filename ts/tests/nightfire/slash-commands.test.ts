import { describe, expect, it } from "vitest";
import {
  buildNightfireSlashCommands,
  filterNightfireSlashCommands,
  findNightfireSlashMatch,
  removeNightfireSlashText
} from "../../src/nightfire/slash-commands";

describe("nightfire/slash-commands", () => {
  it("builds default commands and merges custom aliases for registered block types", () => {
    const commands = buildNightfireSlashCommands(
      [
        { type: "markdown", label: "Markdown" },
        { type: "media", label: "Media" }
      ],
      {
        enabled: true,
        commands: [
          {
            type: "media",
            aliases: ["image"],
            keywords: ["photo"]
          },
          {
            type: "unknown",
            label: "Ignored"
          }
        ]
      }
    );

    expect(commands).toEqual([
      expect.objectContaining({
        type: "markdown",
        label: "Markdown"
      }),
      expect.objectContaining({
        type: "media",
        aliases: expect.arrayContaining(["media", "image"]),
        keywords: expect.arrayContaining(["photo"])
      })
    ]);
  });

  it("filters commands across labels, aliases, and keywords", () => {
    const commands = [
      {
        id: "insert-markdown",
        type: "markdown",
        label: "Markdown",
        description: "Insert a markdown block.",
        aliases: ["paragraph"],
        keywords: ["text"]
      },
      {
        id: "insert-media",
        type: "media",
        label: "Media",
        description: "Insert a media block.",
        aliases: ["image"],
        keywords: ["photo"]
      }
    ];

    expect(filterNightfireSlashCommands(commands, "para")).toEqual([commands[0]]);
    expect(filterNightfireSlashCommands(commands, "photo")).toEqual([commands[1]]);
  });

  it("detects slash tokens at the caret and removes them after selection", () => {
    const context = {
      value: "Intro /med",
      selectionStart: 10,
      selectionEnd: 10
    };

    const match = findNightfireSlashMatch(context);
    expect(match).toEqual({
      start: 6,
      end: 10,
      query: "med"
    });
    expect(removeNightfireSlashText(context.value, match!)).toBe("Intro ");
  });

  it("ignores selections and non-command slashes", () => {
    expect(findNightfireSlashMatch({
      value: "https://example.com",
      selectionStart: 19,
      selectionEnd: 19
    })).toBeNull();

    expect(findNightfireSlashMatch({
      value: "/media",
      selectionStart: 0,
      selectionEnd: 6
    })).toBeNull();
  });
});
