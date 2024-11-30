import { createSignal } from "solid-js";
import { Movie } from "../types";

export function MovieForm({
  submit,
  defaultValues,
}: {
  submit: (data: Movie) => Promise<void>;
  defaultValues?: Movie;
}) {
  const [title, setTitle] = createSignal(defaultValues?.title ?? "");
  const [popularity, setPopularity] = createSignal(
    defaultValues?.popularity ?? 0
  );
  const [voteAverage, setVoteAverage] = createSignal(
    defaultValues?.vote_average ?? 0
  );
  const [releaseDate, setReleaseDate] = createSignal(
    defaultValues?.release_date ?? ""
  );
  const [genres, setGenres] = createSignal(defaultValues?.genres ?? "");
  const [keywords, setKeywords] = createSignal(defaultValues?.keywords ?? "");
  const [budget, setBudget] = createSignal(defaultValues?.budget ?? 0);
  const [revenue, setRevenue] = createSignal(defaultValues?.revenue ?? 0);
  const [voteCount, setVoteCount] = createSignal(
    defaultValues?.vote_count ?? 0
  );
  const [overview, setOverview] = createSignal(defaultValues?.overview ?? "");

  async function handleSubmit() {
    await submit({
      id: defaultValues?.id ?? 0,
      title: title(),
      popularity: popularity(),
      vote_average: voteAverage(),
      release_date: releaseDate(),
      genres: genres(),
      keywords: keywords(),
      budget: budget(),
      revenue: revenue(),
      vote_count: voteCount(),
      overview: overview(),
    });
  }

  return (
    <>
      <div class="flex flex-col gap-2 text-white text-sm overflow-auto">
        <label class="flex flex-col gap-1">
          <span>Title</span>
          <input
            type="text"
            value={title()}
            onChange={(e) => setTitle(e.currentTarget.value)}
            class="border-[1px] border-zinc-700 bg-neutral-800 p-2 rounded-md"
          />
        </label>
        <label class="flex flex-col gap-1">
          <span>Popularity</span>
          <input
            type="number"
            value={popularity()}
            onChange={(e) => setPopularity(Number(e.currentTarget.value))}
            class="border-[1px] border-zinc-700 bg-neutral-800 text-white p-2 rounded-md"
          />
        </label>
        <label class="flex flex-col gap-1">
          <span>Vote Average</span>
          <input
            type="number"
            value={voteAverage()}
            onChange={(e) => setVoteAverage(Number(e.currentTarget.value))}
            class="border-[1px] border-zinc-700 bg-neutral-800 text-white p-2 rounded-md"
          />
        </label>
        <label class="flex flex-col gap-1">
          <span>Release Date</span>
          <input
            type="date"
            value={releaseDate()}
            onChange={(e) => setReleaseDate(e.currentTarget.value)}
            class="border-[1px] border-zinc-700 bg-neutral-800 text-white p-2 rounded-md"
          />
        </label>
        <label class="flex flex-col gap-1">
          <span>Genres</span>
          <input
            type="text"
            value={genres()}
            onChange={(e) => setGenres(e.currentTarget.value)}
            class="border-[1px] border-zinc-700 bg-neutral-800 text-white p-2 rounded-md"
          />
        </label>
        <label class="flex flex-col gap-1">
          <span>Keywords</span>
          <input
            type="text"
            value={keywords()}
            onChange={(e) => setKeywords(e.currentTarget.value)}
            class="border-[1px] border-zinc-700 bg-neutral-800 text-white p-2 rounded-md"
          />
        </label>
        <label class="flex flex-col gap-1">
          <span>Budget</span>
          <input
            type="number"
            value={budget()}
            onChange={(e) => setBudget(Number(e.currentTarget.value))}
            class="border-[1px] border-zinc-700 bg-neutral-800 text-white p-2 rounded-md"
          />
        </label>
        <label class="flex flex-col gap-1">
          <span>Revenue</span>
          <input
            type="number"
            value={revenue()}
            onChange={(e) => setRevenue(Number(e.currentTarget.value))}
            class="border-[1px] border-zinc-700 bg-neutral-800 text-white p-2 rounded-md"
          />
        </label>
        <label class="flex flex-col gap-1">
          <span>Vote Count</span>
          <input
            type="number"
            value={voteCount()}
            onChange={(e) => setVoteCount(Number(e.currentTarget.value))}
            class="border-[1px] border-zinc-700 bg-neutral-800 text-white p-2 rounded-md"
          />
        </label>
        <label class="flex flex-col gap-1">
          <span>Overview</span>
          <textarea
            value={overview()}
            onChange={(e) => setOverview(e.currentTarget.value)}
            class="border-[1px] border-zinc-700 bg-neutral-800 text-white p-2 rounded-md"
          />
        </label>
      </div>
      <div class="flex justify-center text-white flex-1">
        <button
          class="border-[1px] text-sm w-full border-zinc-700 bg-neutral-800 px-2 py-1 rounded-md hover:opacity-75 active:opacity-50"
          onClick={handleSubmit}
        >
          Submit
        </button>
      </div>
    </>
  );
}
