SQL_FILES = $(shell find pop_content/migrations/ -name '*.sql') \
	$(shell find pop_content/queries/ -name '*.sql') \
	$(shell find pop_wiki/migrations/ -name '*.sql') \
	$(shell find postprocess_wiki/ -name '*.sql')
SQL_MARKERS = $(SQL_FILES:%=%.marker)

CORNUCOPIA_SRC = pop_content/src/cornucopia.rs

.PHONY: docker
docker:
	cd docker && docker image build -t reddit .

.PHONY: db
db:
	sudo service postgresql start
	sudo su - postgres -c "psql \
	-c \"CREATE DATABASE reddit;\" \
	-c \"CREATE USER root PASSWORD '1234';\" \
	-c \"GRANT ALL PRIVILEGES ON DATABASE reddit TO root;\" \
	-c \"ALTER DATABASE reddit OWNER TO root;\""

.PHONY: db-content
db-content: db
	cd pop_content && refinery migrate -c ../refinery.toml

.PHONY: db-wiki
db-wiki: db
	cd pop_wiki && refinery migrate -c ../refinery.toml

.PHONY: cornucopia
cornucopia: $(CORNUCOPIA_SRC)

$(CORNUCOPIA_SRC): $(SQL_FILES)
	$(MAKE) db-content
	cd pop_content && cornucopia --serialize live postgres://root:1234@localhost:5432/reddit
	rustfmt --edition 2021 $(CORNUCOPIA_SRC)

%.sql.marker: %.sql
	pg_format --type-case 2 --function-case 1 --no-space-function --inplace $<
	touch $@

.PHONY: sql-fmt
sql-fmt: $(SQL_MARKERS)

.PHONY: clean
clean:
	-rm $(SQL_MARKERS)
	-cd pop_content && cargo clean
