CREATE TABLE public.command_history (
	id serial4 NOT NULL primary key,
	user_id varchar NOT NULL,
	command_name varchar NULL,
	executed_at timestamptz NOT NULL
);
CREATE INDEX command_history_command_name_idx ON public.command_history USING btree (command_name);
CREATE INDEX command_history_id_idx ON public.command_history USING btree (id);
CREATE INDEX command_history_user_id_idx ON public.command_history USING btree (user_id);
