insert into jig (id, creator_id, author_id, created_at, language)
values
    ('0cc084bc-7c83-11eb-9f77-e3218dffb008', '1f241e1b-b537-493f-a230-075cb16315be', '1f241e1b-b537-493f-a230-075cb16315be', '2021-03-04 00:46:26.134651+00', 'en');


insert into jig_module (jig_id, id, index, kind, contents, created_at, is_complete)
values
    ('0cc084bc-7c83-11eb-9f77-e3218dffb008', '0cbfdd82-7c83-11eb-9f77-d7d86264c3bc', 0, 0, '{}', '2021-03-04 00:46:26.134651+00', false),
    ('0cc084bc-7c83-11eb-9f77-e3218dffb008', '0cc03a02-7c83-11eb-9f77-f77f9ad65e9a', 1, null, null, '2021-03-04 00:46:26.134651+00', false);
