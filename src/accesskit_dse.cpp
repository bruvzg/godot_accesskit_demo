#include <godot_cpp/core/binder_common.hpp>
#include <godot_cpp/core/class_db.hpp>

#include <godot_cpp/classes/display_server.hpp>
#include <godot_cpp/classes/display_server_extension.hpp>
#include <godot_cpp/classes/display_server_extension_manager.hpp>
#include <godot_cpp/classes/global_constants.hpp>

#include <godot_cpp/templates/hash_map.hpp>

#include <godot_cpp/variant/utility_functions.hpp>

using namespace godot;

typedef void (*AccessKitAction)(void *, const char *);

// Functions exported by AccessKit library.
extern "C" void *accesskit_init(void *p_dse, void *p_native_display_handle, void *p_native_window_handle, void *p_native_view_handle, AccessKitAction p_action, const char *p_update, const char *p_app_name);
extern "C" void accesskit_destroy(void *p_dse, void *p_adapter);
extern "C" bool accesskit_push_update(void *p_dse, void *p_adapter, const char *p_update);

// DisplayServer extension.
class AccessKitDSE : public DisplayServerExtension {
	GDCLASS(AccessKitDSE, DisplayServerExtension);

	struct AdapterData {
		int64_t display_handle = 0;
		int64_t window_handle = 0;
		int64_t view_handle = 0;
		void *adapter = nullptr;

		AdapterData() {}
		AdapterData(int64_t p_native_display_handle, int64_t p_window_handle, int64_t p_view_handle, void *p_adapter) {
			display_handle = p_native_display_handle;
			window_handle = p_window_handle;
			view_handle = p_view_handle;
			adapter = p_adapter;
		}
	};

	HashMap<int32_t, AdapterData> window_adapters;

protected:
	static void _bind_methods() {
		ClassDB::bind_method(D_METHOD("update_tree", "id", "update"), &AccessKitDSE::update_tree);

		ADD_SIGNAL(MethodInfo("action_signal", PropertyInfo(Variant::STRING, "json_data")));
	}

public:
	// Callback function for AccessKit library.
	static void accesskit_action(void *p_dse, const char *p_json_data) {
		String json_data = String::utf8(p_json_data);
		UtilityFunctions::print("[!] callback ", json_data);
		((AccessKitDSE *)p_dse)->emit_signal("action_signal", json_data);
	}

	virtual String _get_name() const override {
		return String("AccessKit");
	}

	virtual void _create_window(int32_t p_window_id, int64_t p_native_display_handle, int64_t p_native_window_handle, int64_t p_native_view_handle) override {
		UtilityFunctions::print("[!] creare window ", p_window_id);

		void *adapter = accesskit_init(this, (void *)p_native_display_handle, (void *)p_native_window_handle, (void *)p_native_view_handle, accesskit_action, R"JSON({
				"nodes":[
					[1, {"role":"window","name":"Hello from Godot"}]
				],
				"focus":1,
				"tree":{"root":1}
			}
		)JSON",
				"Godot");

		if (adapter) {
			window_adapters[p_window_id] = AdapterData(p_native_display_handle, p_native_window_handle, p_native_view_handle, adapter);
		} else {
			UtilityFunctions::print("[!] creare window failed ", p_window_id);
		}
	}

	virtual void _destroy_window(int32_t p_window_id, int64_t p_native_display_handle, int64_t p_native_window_handle, int64_t p_native_view_handle) override {
		UtilityFunctions::print("[!] destroy window ", p_window_id);
		if (window_adapters.has(p_window_id)) {
			ERR_FAIL_COND(window_adapters[p_window_id].display_handle != p_native_display_handle);
			ERR_FAIL_COND(window_adapters[p_window_id].window_handle != p_native_window_handle);
			ERR_FAIL_COND(window_adapters[p_window_id].view_handle != p_native_view_handle);

			accesskit_destroy(this, window_adapters[p_window_id].adapter);
			window_adapters.erase(p_window_id);
		}
	}

	bool update_tree(int32_t p_window_id, const String &p_update) {
		UtilityFunctions::print("[!] update ", p_window_id, " ", p_update);
		if (window_adapters.has(p_window_id)) {
			return accesskit_push_update(this, window_adapters[p_window_id].adapter, p_update.utf8().get_data());
		} else {
			return false;
		}
	}

	AccessKitDSE() {
		UtilityFunctions::print("[!] dse init");
	}

	~AccessKitDSE() {
		UtilityFunctions::print("[!] dse uninit");
		for (const KeyValue<int32_t, AdapterData> &E : window_adapters) {
			accesskit_destroy(this, E.value.adapter);
		}
	}
};

// GDExtension init.
void initialize_accesskit_module(ModuleInitializationLevel p_level) {
	if (p_level != MODULE_INITIALIZATION_LEVEL_SERVERS) {
		return;
	}

	UtilityFunctions::print("[!] module init");

	GDREGISTER_CLASS(AccessKitDSE);
	DisplayServerExtensionManager *dseman = DisplayServerExtensionManager::get_singleton();
	if (dseman) {
		Ref<AccessKitDSE> dse;
		dse.instantiate();
		dseman->add_interface(dse);
	}
}

void uninitialize_accesskit_module(ModuleInitializationLevel p_level) {
	if (p_level != MODULE_INITIALIZATION_LEVEL_SERVERS) {
		return;
	}

	UtilityFunctions::print("[!] module uninit");
}

extern "C" GDExtensionBool GDE_EXPORT accesskit_library_init(const GDExtensionInterface *p_interface, GDExtensionClassLibraryPtr p_library, GDExtensionInitialization *r_initialization) {
	godot::GDExtensionBinding::InitObject init_obj(p_interface, p_library, r_initialization);

	init_obj.register_initializer(initialize_accesskit_module);
	init_obj.register_terminator(uninitialize_accesskit_module);
	init_obj.set_minimum_library_initialization_level(MODULE_INITIALIZATION_LEVEL_SERVERS);

	return init_obj.init();
}
