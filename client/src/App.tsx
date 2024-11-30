import type { Component } from "solid-js";
import { AiOutlineSearch } from "solid-icons/ai";
import {
  createEffect,
  createMemo,
  createSignal,
  For,
  onMount,
  Show,
} from "solid-js";
import { RiSystemAddCircleLine } from "solid-icons/ri";
import axios from "axios";
import {
  createSolidTable,
  flexRender,
  getCoreRowModel,
} from "@tanstack/solid-table";
import Drawer from "@corvu/drawer";
import Dialog from "@corvu/dialog";
import { MovieForm } from "../components/MovieForm";
import { Movie } from "../types";

const App: Component = () => {
  const columns = createMemo(() => [
    { header: "Title", accessorKey: "title" },
    { header: "Popularity", accessorKey: "popularity" },
    { header: "Vote Average", accessorKey: "vote_average" },
    { header: "Release Date", accessorKey: "release_date" },
    { header: "Genres", accessorKey: "genres" },
    { header: "Keywords", accessorKey: "keywords" },
    { header: "Budget", accessorKey: "budget" },
    { header: "Revenue", accessorKey: "revenue" },
    { header: "Vote Count", accessorKey: "vote_count" },
    { header: "Overview", accessorKey: "overview" },
  ]);
  const [selectedMovie, setSelectedMovie] = createSignal<Movie | null>(null);
  const [searchResults, setSearchResults] = createSignal<Movie[]>([]);
  const [movies, setMovies] = createSignal<Movie[]>([]);
  const [numEntries, setNumEntries] = createSignal(20);
  const [page, setPage] = createSignal(0);

  const [isUpdateFormOpen, setIsUpdateFormOpen] = createSignal(false);
  const [isAddFormOpen, setIsAddFormOpen] = createSignal(false);
  const [displayDelDialog, setDisplayDelDialog] = createSignal(false);

  const [search, setSearch] = createSignal("");
  const [query, setQuery] = createSignal("");

  const table = createSolidTable({
    columns: columns(),
    getCoreRowModel: getCoreRowModel(),
    get data() {
      return search() === "" ? movies() : searchResults();
    },
  });

  const apiEndpoint = "http://localhost:8000";

  onMount(async () => {
    await fetchMovies(numEntries());
  });

  createEffect(async () => {
    await fetchMovies(numEntries());
  });

  createEffect(async () => {
    if (search() !== "") {
      await searchMovies(search(), numEntries());
    }
  });

  async function fetchMovies(entries: number) {
    const start = entries * page();
    const route = `/movies/get/range/${start}/${start + entries}`;
    try {
      const response = await axios.get(`${apiEndpoint}${route}`);
      if (response.data.length === 0) {
        setPage(page() - 1);
        return;
      }
      setMovies(response.data);
    } catch (error) {
      console.error(error);
    }
  }

  async function searchMovies(query: string, entries: number) {
    const route = `/movies/search/${query.replace(" ", "%20")}/${entries}`;
    try {
      const response = await axios.get(`${apiEndpoint}${route}`);
      setSearchResults(response.data);
    } catch (error) {
      console.error;
    }
  }

  async function handleUpdateMovie(movie: Movie) {
    const { id, ...rest } = movie;
    const route = `/movies/upd/${id}`;
    try {
      await axios.post(`${apiEndpoint}${route}`, rest, {
        headers: {
          "Content-Type": "application/json",
        },
      });
      await fetchMovies(numEntries());
      setIsUpdateFormOpen(false);
    } catch (error) {
      alert("Failed to update movie. Please try again.");
      console.error(error);
    }
  }

  async function handleAddMovie(movie: Movie) {
    try {
      await axios.post(`${apiEndpoint}/movies/ins`, movie, {
        headers: {
          "Content-Type": "application/json",
        },
      });
      await fetchMovies(numEntries());
      setIsAddFormOpen(false);
    } catch (error) {
      alert("Failed to add movie. Please try again.");
      console.error(error);
    }
  }

  async function handleDelete() {
    if (!selectedMovie()) return;
    const route = `/movies/del/${selectedMovie().id}`;
    try {
      await axios.post(`${apiEndpoint}${route}`);
      await fetchMovies(numEntries());
      setDisplayDelDialog(false);
    } catch (error) {
      alert("Failed to delete movie. Please try again.");
      console.error(error);
    }
  }

  return (
    <main class="min-h-[100svh] flex flex-col bg-neutral-950">
      <nav class="flex py-5 px-10 w-[100%] justify-between items-center gap-4">
        <div
          onClick={() => {
            setQuery("");
            setSearch("");
          }}
          class="flex items-baseline justify-center max-w-[256px] overflow-hidden cursor-pointer"
        >
          <img
            src="/trail.png"
            class="w-[72px] h-auto translate-x-[64px] translate-y-[10px] z-20"
          />
          <img
            src="/popcorn.png"
            alt="popcorn"
            class="w-9 h-auto -rotate-[8deg] translate-x-[9px] z-10"
          />
          <p class="bangers text-5xl z-10 text-borders">
            <span class="text-red-500">
              C<span class="text-white">i</span>
              <span class="text-red-400">ne</span>
              ma
            </span>
            <span class="text-white">
              D<span class="text-red-400">B</span>
            </span>
          </p>
          <img
            src="/popcorn.png"
            alt="popcorn"
            class="w-9 h-auto rotate-[4deg] -translate-x-1"
          />
          <img
            src="/trail.png"
            class="w-[72px] h-auto -translate-x-[64px] translate-y-[10px] z-20"
          />
        </div>
        <div class="flex flex-1 max-w-[700px]">
          <input
            type="text"
            value={query()}
            onChange={(e) => setQuery(e.currentTarget.value)}
            class="w-full h-10 px-4 rounded-tl-md rounded-bl-md outline-none border-zinc-700 border-l-[1px] border-t-[1px] border-b-[1px] bg-neutral-800 text-white placeholder:text-zinc-600"
            placeholder="Some super special movie..."
          />
          <button
            onClick={() => {
              if (query().trim() === "") return;
              setSearch(query());
            }}
            class="flex items-center gap-2 rounded-tr-md rounded-br-md px-3 py-1 text-sm h-10 border-zinc-700 border-[1px] hover:opacity-75 active:opacity-50 bg-neutral-800 text-white"
          >
            <AiOutlineSearch size={24} color="white" />
          </button>
        </div>
        <div>
          <button
            onClick={() => setIsAddFormOpen(true)}
            class="flex items-center gap-2 rounded-md px-3 py-1 text-sm h-10 border-zinc-700 border-[1px] hover:opacity-75 active:opacity-50 bg-neutral-800 text-white"
          >
            <RiSystemAddCircleLine color="white" size={16} />
            <p>Add data</p>
          </button>
        </div>
      </nav>
      <div class="px-14 pb-8 pt-4 flex flex-col gap-4">
        <div class="flex text-white justify-between items-center">
          <div class="flex gap-2">
            <p>Show</p>
            <select
              class="border-[1px] border-zinc-700 bg-neutral-800 text-white"
              value={numEntries()}
              onInput={(e) => setNumEntries(Number(e.currentTarget.value))}
            >
              <option value="10">10</option>
              <option value="20">20</option>
              <option value="30">30</option>
            </select>
            <p>entries</p>
          </div>
          <div class="flex gap-2">
            <Show when={selectedMovie()}>
              <button
                onClick={() => setIsUpdateFormOpen(true)}
                class="border-[1px] text-sm border-zinc-700 bg-neutral-800 px-2 py-1 rounded-md hover:opacity-75 active:opacity-50"
              >
                Edit
              </button>
              <button
                onClick={() => setDisplayDelDialog(true)}
                class="border-[1px] text-sm border-zinc-700 bg-neutral-800 px-2 py-1 rounded-md hover:opacity-75 active:opacity-50"
              >
                Delete
              </button>
            </Show>
          </div>
          <Show when={search() !== ""}>
            <div class="text-sm">
              <button
                onClick={() => {
                  setSearch("");
                  setQuery("");
                }}
                class="border-[1px] border-zinc-700 bg-neutral-800 px-2 py-1 rounded-md hover:opacity-75 active:opacity-50"
              >
                Home
              </button>
            </div>
          </Show>
        </div>
        <Show when={searchResults().length < 1 && search() !== ""}>
          <div class="flex flex-1 justify-center items-center text-white">
            <p>No entries found.</p>
          </div>
        </Show>
        <Show
          when={
            (movies().length > 0 && search() === "") ||
            searchResults().length > 0
          }
        >
          <table class="text-white relative rounded-md block overflow-auto max-h-[500px]">
            <For each={table.getHeaderGroups()}>
              {(headerGroup) => (
                <tr
                  class="sticky top-0 bg-zinc-700 text-center"
                  style={{ "box-shadow": "0px 0px 0px 1px gray" }}
                >
                  <td class="px-4" />
                  <For each={headerGroup.headers}>
                    {(header) => (
                      <td class="px-4 flex-1">
                        {header.isPlaceholder
                          ? null
                          : flexRender(
                              header.column.columnDef.header,
                              header.getContext()
                            )}
                      </td>
                    )}
                  </For>
                </tr>
              )}
            </For>
            <For each={table.getRowModel().rows}>
              {(row) => (
                <tr>
                  <td class="border-[1px] border-zinc-700 px-2 py-2">
                    <input
                      type="checkbox"
                      checked={selectedMovie()?.id === row.original.id}
                      onChange={() => {
                        if (selectedMovie()?.id === row.original.id) {
                          setSelectedMovie(null);
                        } else {
                          setSelectedMovie(row.original);
                        }
                      }}
                    />
                  </td>
                  <For each={row.getVisibleCells()}>
                    {(cell) => (
                      <td class="border-[1px] border-zinc-700 px-2 py-2">
                        {flexRender(
                          cell.column.columnDef.cell,
                          cell.getContext()
                        )}
                      </td>
                    )}
                  </For>
                </tr>
              )}
            </For>
          </table>
        </Show>
        <Show when={search() === ""}>
          <div class="flex flex-1 text-white gap-4 justify-center items-center text-xs">
            <button
              class="border-[1px] border-zinc-700 bg-neutral-800 px-2 py-1 rounded-md"
              onClick={() => {
                if (page() <= 0) return;
                setPage(page() - 1);
              }}
            >
              Prev
            </button>
            <p>Page {page() + 1}</p>
            <button
              class="border-[1px] border-zinc-700 bg-neutral-800 px-2 py-1 rounded-md"
              onClick={() => setPage(page() + 1)}
            >
              Next
            </button>
          </div>
        </Show>
      </div>
      <Drawer
        side="right"
        open={isUpdateFormOpen()}
        onOpenChange={(e) => setIsUpdateFormOpen(e.valueOf)}
      >
        <Drawer.Portal>
          <Drawer.Overlay
            class="fixed inset-0 bg-black bg-opacity-50 z-50"
            onClick={() => setIsUpdateFormOpen(false)}
          />
          <Drawer.Content class="fixed inset-y-0 right-0 z-50 flex w-[300px] h-full flex-col bg-neutral-800 transition-transform transform translate-x-full">
            <div class="bg-gray p-4 flex flex-col gap-4">
              <Drawer.Label class="text-lg font-bold text-white">
                Edit Movie
              </Drawer.Label>
              <MovieForm
                submit={handleUpdateMovie}
                defaultValues={selectedMovie()}
              />
            </div>
          </Drawer.Content>
        </Drawer.Portal>
      </Drawer>
      <Drawer
        side="right"
        open={isAddFormOpen()}
        onOpenChange={(e) => setIsAddFormOpen(e.valueOf)}
      >
        <Drawer.Portal>
          <Drawer.Overlay
            class="fixed inset-0 bg-black bg-opacity-50 z-50"
            onClick={() => setIsAddFormOpen(false)}
          />
          <Drawer.Content class="fixed inset-y-0 right-0 z-50 flex w-[300px] h-full flex-col bg-neutral-800 transition-transform transform translate-x-full">
            <div class="bg-gray p-4 flex flex-col gap-4">
              <Drawer.Label class="text-lg font-bold text-white">
                Add Movie
              </Drawer.Label>
              <MovieForm submit={handleAddMovie} />
            </div>
          </Drawer.Content>
        </Drawer.Portal>
      </Drawer>
      <Dialog open={displayDelDialog()} onOpenChange={setDisplayDelDialog}>
        <Dialog.Portal>
          <Dialog.Overlay
            onClick={() => setDisplayDelDialog(false)}
            class="fixed inset-0 bg-black bg-opacity-50 z-50"
          />
          <Dialog.Content class="fixed z-50 flex gap-4 flex-col items-center w-[300px] p-4 bg-neutral-800 rounded-md shadow-lg transition-transform transform -translate-x-1/2 -translate-y-1/2 top-1/2 left-1/2">
            <Dialog.Label class="text-white font-bold">
              Are you sure you want to delete this movie?
            </Dialog.Label>
            <div class="text-sm flex gap-2 justify-end text-white w-full">
              <button
                onClick={() => setDisplayDelDialog(false)}
                class="border-[1px] border-zinc-700 bg-neutral-800 text-white p-2 rounded-md hover:opacity-75 active:opacity-50"
              >
                Close
              </button>
              <button onClick={handleDelete} class="border-[1px] border-zinc-700 bg-red-500 text-white p-2 rounded-md hover:opacity-75 active:opacity-50">
                Delete
              </button>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog>
    </main>
  );
};

export default App;
