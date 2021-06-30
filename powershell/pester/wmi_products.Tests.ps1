Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_products table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_products = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_products' }
				$wmi_products | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_products = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_products' }
				$schema = Get-Content $wmi_products.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_products = Get-Siq 'wmi_products' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_products' }).FullName -Raw

				$json = $wmi_products | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
