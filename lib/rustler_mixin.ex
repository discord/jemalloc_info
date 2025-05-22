defmodule JemallocInfo.RustlerMixin do
  defmacro __using__(_) do
    quote do
      @type jemalloc_allocation_data :: %{
              :epoch => non_neg_integer(),
              :active => non_neg_integer(),
              :allocated => non_neg_integer(),
              :mapped => non_neg_integer(),
              :metadata => non_neg_integer(),
              :resident => non_neg_integer(),
              :retained => non_neg_integer()
            }

      @spec jemalloc_allocation_info() :: {:ok, jemalloc_allocation_data} | {:error, any}
      def jemalloc_allocation_info() do
        :erlang.nif_error(:nif_not_loaded)
      end
    end
  end
end
