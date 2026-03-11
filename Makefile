.PHONY: up down cleanup

up:
	$(MAKE) -C web/backend/rust_kratos infra-up
	$(MAKE) -C web/backend/rust_kratos up
	$(MAKE) -C frontend up

down:
	$(MAKE) -C web/backend/rust_kratos infra-down
	$(MAKE) -C web/backend/rust_kratos down
	$(MAKE) -C web/frontend down

cleanup:
	$(MAKE) -C infrastructure/local_development cleanup
