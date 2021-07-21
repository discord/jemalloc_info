defmodule JemallocInfo.RustlerMixin do
  defmacro __using__(_) do
    quote do
      @type jemalloc_allocation_data :: %{
              :epoch => integer,
              :active => integer,
              :allocated => integer,
              :mapped => integer,
              :metadata => integer,
              :resident => integer,
              :retained => integer
            }

      @spec jemalloc_allocation_info() :: {:ok, jemalloc_allocation_data} | {:error, any}
      def jemalloc_allocation_info() do
        :erlang.nif_error(:nif_not_loaded)
      end
    end
  end
end
