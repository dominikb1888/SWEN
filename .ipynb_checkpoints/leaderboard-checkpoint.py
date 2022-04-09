import pandas as pd
from github import Github

gh = Github("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")


def get_table():
    table = []
    for repo in gh.get_organization("DB-Teaching").get_repos():
        table.append(
            {
                "name": repo.name,
                "user": repo.name.split("-")[-1],
                "url": repo.url,
                "commits": sum([1 for commit in repo.get_commits()]),
                "runs": repo.get_workflow_runs().totalCount,
                "completed": sum(
                    [1 for w in repo.get_workflow_runs() if w.conclusion == "success"]
                ),
                "failed": sum(
                    [1 for w in repo.get_workflow_runs() if w.conclusion != "failure"]
                ),
            }
        )
    return table


# update from gh
# df = pd.DataFrame(table)
# df.to_csv("leaderboard.csv")
def print_table():
    df = pd.read_csv("leaderboard.csv")
    df[["session", "exercise"]] = df["name"].str.split("-|_", expand=True)[[0, 1]]
    # df.pivot_table(
    #     index=["user"],
    #     values=["completed"],
    #     columns=[["session", "excercise"]],
    #     aggfunc="count",
    # )
    df = df.iloc[119:]
    print(
        df[["user", "exercise", "session", "commits", "failed", "completed"]]
        .fillna(0)
        .pivot_table(
            values=["commits", "completed", "failed"],
            index="user",
            aggfunc=sum,
            columns=["session", "exercise"],
            fill_value=0,
            margins=True,
            margins_name="Total",
        )
        # .sort_values(by="Total", ascending=False)
        # .apply(lambda row: "".join(["\u258F" * i for i in row]), axis=1)
        .to_markdown()
    )


print_table()
